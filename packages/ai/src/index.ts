/**
 * @zenvu/ai â€” Client-side AI hooks and utilities
 */

export interface Message {
  id: string;
  role: 'system' | 'user' | 'assistant' | 'tool';
  content: string;
  createdAt: Date;
}

export interface UseChatOptions {
  api?: string;
  initialMessages?: Message[];
  onFinish?: (message: Message) => void;
  onError?: (error: Error) => void;
}

/**
 * A reactive hook-like function for building chat interfaces.
 * Usage: const { messages, input, setInput, handleSubmit } = useChat();
 */
export function useChat(options: UseChatOptions = {}) {
  // In a real framework, these would be bound to the reactive engine (store)
  const api = options.api || '/api/chat';
  let messages: Message[] = options.initialMessages || [];
  let input = '';
  let isLoading = false;

  // Reactivity hook-in points would go here
  // We use generic callbacks for the framework integration

  const setInput = (val: string) => { input = val; };
  
  const append = async (message: Message | { role: 'user', content: string }) => {
    isLoading = true;
    
    const userMsg: Message = {
      id: Math.random().toString(36).substring(7),
      role: message.role,
      content: message.content,
      createdAt: new Date(),
    };
    
    messages = [...messages, userMsg];
    
    try {
      const response = await fetch(api, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: messages.map(m => ({ role: m.role, content: m.content })) }),
      });
      
      if (!response.ok) throw new Error(response.statusText);
      
      // Parse SSE stream
      const reader = response.body?.getReader();
      const decoder = new TextDecoder();
      
      let assistantMsgContent = '';
      const assistantMsgId = Math.random().toString(36).substring(7);
      
      messages = [...messages, { id: assistantMsgId, role: 'assistant', content: '', createdAt: new Date() }];
      
      if (reader) {
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          
          const chunk = decoder.decode(value);
          assistantMsgContent += chunk;
          
          // Update the last message
          messages[messages.length - 1].content = assistantMsgContent;
          // Trigger framework reactivity here
        }
      }
      
      if (options.onFinish) {
        options.onFinish(messages[messages.length - 1]);
      }
      
    } catch (err: any) {
      if (options.onError) options.onError(err);
      console.error('[Zenvu AI] Chat error:', err);
    } finally {
      isLoading = false;
    }
  };

  const handleSubmit = (e?: any) => {
    if (e?.preventDefault) e.preventDefault();
    if (!input.trim()) return;
    
    const content = input;
    input = '';
    append({ role: 'user', content });
  };

  return {
    get messages() { return messages; },
    get input() { return input; },
    get isLoading() { return isLoading; },
    setInput,
    handleSubmit,
    append,
  };
}
