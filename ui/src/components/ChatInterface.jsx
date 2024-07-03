import React, { useState, useRef, useEffect, useCallback } from 'react';
import './ChatInterface.css';
import { exampleAnswer } from './samplePrompt';

const ChatInterface = () => {
  const [input, setInput] = useState('');
  const [ragEnabled, setRagEnabled] = useState(false);
  const [conversation, setConversation] = useState([]);
  const textareaRef = useRef(null);

  useEffect(() => {
    adjustTextareaHeight();
  }, [input]);

  const adjustTextareaHeight = () => {
    const textarea = textareaRef.current;
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = `${textarea.scrollHeight}px`;
    }
  };

  const copyToClipboard = useCallback((text) => {
    navigator.clipboard.writeText(text).then(() => {
      console.log('Code copied to clipboard');
    }).catch(err => {
      console.error('Failed to copy code: ', err);
    });
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (input.trim() === '') return;
  
    const newQuestion = { type: 'question', content: input };
    setConversation(prev => [...prev, newQuestion]);
  
    console.log('Sending prompt:', input, 'RAG enabled:', ragEnabled);
  
    const payload = {
      conversation_id: 0,
      model: 'claude-3-5-sonnet-20240620',
      prompt: input,
    };
  
    try {
      const response = await fetch('http://localhost:8080/talk_to_kinode:talk_to_kinode:uncentered.os/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });
  
      if (response.ok) {
        const data = await response.json();
        const updatedConversation = data.map((content, index) => ({
          type: index % 2 === 0 ? 'question' : 'answer',
          content,
        }));
        setConversation(updatedConversation);
      } else {
        console.error('Failed to send prompt:', response.statusText);
      }
    } catch (error) {
      console.error('Error sending prompt:', error);
    }
  
    setInput('');
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <div className="chat-interface">
      <div className="conversation">
        {conversation.map((message, index) => (
          <div key={index} className={`message ${message.type}`}>
            {message.content.split(/(\`\`\`[\s\S]*?\`\`\`)/).map((part, i) => {
              if (part.startsWith('```') && part.endsWith('```')) {
                const code = part.slice(3, -3);
                return (
                  <pre key={i} onClick={() => copyToClipboard(code)}>
                    <code>{code}</code>
                  </pre>
                );
              }
              return <p key={i}>{part}</p>;
            })}
          </div>
        ))}
      </div>
      <div className="input-area">
        <form onSubmit={handleSubmit}>
          <div className="input-container">
            <textarea
              ref={textareaRef}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Type your message here..."
              className="chat-input"
              rows={1}
            />
            <button type="submit" className="submit-button">Send</button>
          </div>
          <div className="rag-toggle">
            <input
              type="checkbox"
              id="rag-checkbox"
              checked={ragEnabled}
              onChange={(e) => setRagEnabled(e.target.checked)}
            />
            <label htmlFor="rag-checkbox">RAG On/Off</label>
          </div>
        </form>
      </div>
    </div>
  );
};

export default ChatInterface;
