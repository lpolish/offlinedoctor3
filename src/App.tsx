import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { 
  Send, 
  MessageSquare, 
  Download, 
  CheckCircle, 
  AlertCircle, 
  Settings,
  Trash2,
  Plus
} from "lucide-react";
import "./App.css";

// Types
interface ChatMessage {
  id: string;
  role: string;
  content: string;
  timestamp: string;
}

interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
}

interface ModelInfo {
  name: string;
  size: number;
  description: string;
  download_url: string;
  filename: string;
  is_downloaded: boolean;
}

interface ChatRequest {
  message: string;
  conversation_id?: string;
}

interface ChatResponse {
  message: string;
  conversation_id: string;
  message_id: string;
}

function App() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [currentConversation, setCurrentConversation] = useState<string | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputMessage, setInputMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [availableModels, setAvailableModels] = useState<ModelInfo[]>([]);
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [isAiReady, setIsAiReady] = useState(false);
  const [showSetup, setShowSetup] = useState(true);
  const [downloadProgress, setDownloadProgress] = useState<{[key: string]: number}>({});
  
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    initializeApp();
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  const initializeApp = async () => {
    try {
      await invoke("initialize_app");
      const models = await invoke("get_available_models") as ModelInfo[];
      setAvailableModels(models);
      setIsInitialized(true);
      
      // Check if any model is downloaded
      const downloadedModel = models.find(m => m.is_downloaded);
      if (downloadedModel) {
        setSelectedModel(downloadedModel.filename);
        await initializeAI(downloadedModel.filename);
      }
      
      loadConversations();
    } catch (error) {
      console.error("Failed to initialize app:", error);
    }
  };

  const loadConversations = async () => {
    try {
      const convs = await invoke("get_conversations") as Conversation[];
      setConversations(convs);
    } catch (error) {
      console.error("Failed to load conversations:", error);
    }
  };

  const loadConversationMessages = async (conversationId: string) => {
    try {
      const msgs = await invoke("get_conversation_messages", { conversationId }) as ChatMessage[];
      setMessages(msgs);
      setCurrentConversation(conversationId);
    } catch (error) {
      console.error("Failed to load conversation messages:", error);
    }
  };

  const downloadModel = async (model: ModelInfo) => {
    try {
      setDownloadProgress({...downloadProgress, [model.filename]: 0});
      await invoke("download_model", { model });
      
      // Update model list
      const updatedModels = availableModels.map(m => 
        m.filename === model.filename ? {...m, is_downloaded: true} : m
      );
      setAvailableModels(updatedModels);
      setDownloadProgress({...downloadProgress, [model.filename]: 100});
      
      // If no model is selected, select this one
      if (!selectedModel) {
        setSelectedModel(model.filename);
        await initializeAI(model.filename);
      }
    } catch (error) {
      console.error("Failed to download model:", error);
      setDownloadProgress({...downloadProgress, [model.filename]: -1});
    }
  };

  const initializeAI = async (modelFilename: string) => {
    try {
      setIsLoading(true);
      await invoke("initialize_ai_engine", { modelFilename });
      setIsAiReady(true);
      setShowSetup(false);
    } catch (error) {
      console.error("Failed to initialize AI:", error);
      setIsAiReady(false);
    } finally {
      setIsLoading(false);
    }
  };

  const sendMessage = async () => {
    if (!inputMessage.trim() || !isAiReady || isLoading) return;

    const userMessage = inputMessage.trim();
    setInputMessage("");
    setIsLoading(true);

    try {
      const request: ChatRequest = {
        message: userMessage,
        conversation_id: currentConversation || undefined
      };

      const response = await invoke("send_chat_message", { request }) as ChatResponse;
      
      // If this is a new conversation, update the conversation list
      if (!currentConversation) {
        await loadConversations();
        setCurrentConversation(response.conversation_id);
      }
      
      // Reload messages for the conversation
      await loadConversationMessages(response.conversation_id);
      
    } catch (error) {
      console.error("Failed to send message:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const startNewConversation = () => {
    setCurrentConversation(null);
    setMessages([]);
  };

  const deleteConversation = async (conversationId: string) => {
    try {
      await invoke("delete_conversation", { conversationId });
      await loadConversations();
      if (currentConversation === conversationId) {
        startNewConversation();
      }
    } catch (error) {
      console.error("Failed to delete conversation:", error);
    }
  };

  const formatFileSize = (bytes: number) => {
    const gb = bytes / (1024 * 1024 * 1024);
    return `${gb.toFixed(1)} GB`;
  };

  if (!isInitialized) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-100">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Initializing Offline Doctor AI...</p>
        </div>
      </div>
    );
  }

  if (showSetup || !isAiReady) {
    return (
      <div className="h-screen bg-gray-100 flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-lg p-8 max-w-2xl w-full">
          <div className="text-center mb-8">
            <h1 className="text-3xl font-bold text-gray-800 mb-2">Offline Doctor AI</h1>
            <p className="text-gray-600">Medical AI Assistant for Remote Healthcare</p>
          </div>

          <div className="space-y-6">
            <div>
              <h2 className="text-xl font-semibold mb-4 flex items-center">
                <Download className="mr-2" size={20} />
                Available AI Models
              </h2>
              <div className="space-y-3">
                {availableModels.map((model) => (
                  <div key={model.filename} className="border rounded-lg p-4">
                    <div className="flex items-center justify-between mb-2">
                      <h3 className="font-medium">{model.name}</h3>
                      <span className="text-sm text-gray-500">{formatFileSize(model.size)}</span>
                    </div>
                    <p className="text-sm text-gray-600 mb-3">{model.description}</p>
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-2">
                        {model.is_downloaded ? (
                          <span className="flex items-center text-green-600 text-sm">
                            <CheckCircle size={16} className="mr-1" />
                            Downloaded
                          </span>
                        ) : downloadProgress[model.filename] !== undefined ? (
                          downloadProgress[model.filename] === -1 ? (
                            <span className="flex items-center text-red-600 text-sm">
                              <AlertCircle size={16} className="mr-1" />
                              Download Failed
                            </span>
                          ) : (
                            <span className="text-sm text-blue-600">
                              Downloading... {downloadProgress[model.filename]}%
                            </span>
                          )
                        ) : (
                          <button
                            onClick={() => downloadModel(model)}
                            className="bg-blue-600 text-white px-4 py-2 rounded text-sm hover:bg-blue-700"
                          >
                            Download
                          </button>
                        )}
                      </div>
                      {model.is_downloaded && (
                        <button
                          onClick={() => {
                            setSelectedModel(model.filename);
                            initializeAI(model.filename);
                          }}
                          disabled={isLoading}
                          className="bg-green-600 text-white px-4 py-2 rounded text-sm hover:bg-green-700 disabled:opacity-50"
                        >
                          {isLoading && selectedModel === model.filename ? "Starting..." : "Use Model"}
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen flex bg-gray-100">
      {/* Sidebar */}
      <div className="w-80 bg-white border-r border-gray-200 flex flex-col">
        <div className="p-4 border-b border-gray-200">
          <h1 className="text-xl font-bold text-gray-800">Offline Doctor AI</h1>
          <button
            onClick={startNewConversation}
            className="w-full mt-3 bg-blue-600 text-white px-4 py-2 rounded flex items-center justify-center hover:bg-blue-700"
          >
            <Plus size={16} className="mr-2" />
            New Conversation
          </button>
        </div>
        
        <div className="flex-1 overflow-y-auto scrollbar-thin">
          <div className="p-4">
            <h2 className="text-sm font-medium text-gray-500 mb-3">Recent Conversations</h2>
            <div className="space-y-2">
              {conversations.map((conv) => (
                <div
                  key={conv.id}
                  className={`p-3 rounded cursor-pointer group flex items-center justify-between ${
                    currentConversation === conv.id ? 'bg-blue-50 border border-blue-200' : 'hover:bg-gray-50'
                  }`}
                  onClick={() => loadConversationMessages(conv.id)}
                >
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-800 truncate">{conv.title}</p>
                    <p className="text-xs text-gray-500">
                      {new Date(conv.updated_at).toLocaleDateString()}
                    </p>
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteConversation(conv.id);
                    }}
                    className="opacity-0 group-hover:opacity-100 text-red-500 hover:text-red-700 p-1"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="p-4 border-t border-gray-200">
          <button
            onClick={() => setShowSetup(true)}
            className="w-full bg-gray-100 text-gray-700 px-4 py-2 rounded flex items-center justify-center hover:bg-gray-200"
          >
            <Settings size={16} className="mr-2" />
            Model Settings
          </button>
        </div>
      </div>

      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col">
        <div className="bg-white border-b border-gray-200 p-4">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-800">
              {currentConversation ? "Medical Consultation" : "New Consultation"}
            </h2>
            <div className="flex items-center text-sm text-gray-500">
              <CheckCircle size={16} className="mr-1 text-green-500" />
              AI Ready - {selectedModel.replace('-', ' ').replace('.gguf', '')}
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto scrollbar-thin p-4 space-y-4">
          {messages.length === 0 ? (
            <div className="text-center py-12">
              <MessageSquare size={48} className="mx-auto text-gray-300 mb-4" />
              <h3 className="text-lg font-medium text-gray-500 mb-2">Start a Medical Consultation</h3>
              <p className="text-gray-400 max-w-md mx-auto">
                Ask questions about symptoms, conditions, treatments, or general medical inquiries. 
                This AI assistant is designed to help healthcare professionals with educational information.
              </p>
            </div>
          ) : (
            messages.map((message) => (
              <div
                key={message.id}
                className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
              >
                <div
                  className={`max-w-3xl px-4 py-3 rounded-lg ${
                    message.role === 'user'
                      ? 'bg-blue-600 text-white'
                      : 'bg-white border border-gray-200 text-gray-800'
                  }`}
                >
                  <div className="whitespace-pre-wrap">{message.content}</div>
                  <div className={`text-xs mt-2 ${
                    message.role === 'user' ? 'text-blue-100' : 'text-gray-400'
                  }`}>
                    {new Date(message.timestamp).toLocaleTimeString()}
                  </div>
                </div>
              </div>
            ))
          )}
          
          {isLoading && (
            <div className="flex justify-start">
              <div className="bg-white border border-gray-200 rounded-lg px-4 py-3">
                <div className="flex items-center space-x-2">
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"></div>
                  <span className="text-gray-600">AI is thinking...</span>
                </div>
              </div>
            </div>
          )}
          
          <div ref={messagesEndRef} />
        </div>

        <div className="bg-white border-t border-gray-200 p-4">
          <div className="flex space-x-3">
            <input
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && !e.shiftKey && sendMessage()}
              placeholder="Ask about symptoms, conditions, or medical questions..."
              className="flex-1 border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              disabled={isLoading}
            />
            <button
              onClick={sendMessage}
              disabled={!inputMessage.trim() || isLoading}
              className="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
            >
              <Send size={16} />
            </button>
          </div>
          <p className="text-xs text-gray-500 mt-2 text-center">
            AI responses are for educational purposes only. Always consult with qualified medical professionals for clinical decisions.
          </p>
        </div>
      </div>
    </div>
  );
}

export default App;
