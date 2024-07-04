import React, { useState, useRef, useEffect, useCallback } from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import 'react-syntax-highlighter/dist/esm/languages/prism/rust';
import './ChatInterface.css';
import { exampleAnswer } from './samplePrompt';

const ChatInterface = () => {
  const [input, setInput] = useState('');
  const [ragEnabled, setRagEnabled] = useState(true);
  const [conversation, setConversation] = useState([]);
  const [conversationId, setConversationId] = useState(null);
  const textareaRef = useRef(null);
  const [enrichedPrompt, setEnrichedPrompt] = useState('');
  const [isEnriching, setIsEnriching] = useState(false);

  useEffect(() => {
    adjustTextareaHeight();
  }, [input]);

  useEffect(() => {
    startNewConversation();
  }, []);

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [conversation]);

  const startNewConversation = async () => {
    try {
      const response = await fetch('http://localhost:8080/talk_to_kinode:talk_to_kinode:uncentered.os/new_conversation', {
        method: 'POST',
      });
      if (response.ok) {
        const data = await response.json();
        console.log('Data:', data);
        // Use 'id' instead of 'conversation_id'
        setConversationId(data.id);
        console.log('New conversation started with ID:', data.id);
      } else {
        console.error('Failed to start new conversation:', response.statusText);
      }
    } catch (error) {
      console.error('Error starting new conversation:', error);
    }
  };

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
    setInput('');

    console.log('Sending prompt:', input, 'RAG enabled:', ragEnabled);
    console.log('Conversation ID:', conversationId);

    try {
      let enrichedPrompt = '';
      
      if (ragEnabled && conversation.length === 0) {
        setIsEnriching(true);
        const ragResponse = await fetch('http://localhost:8080/talk_to_kinode:talk_to_kinode:uncentered.os/rag', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(input),
        });

        if (!ragResponse.ok) {
          throw new Error('Failed to get RAG response');
        }

        enrichedPrompt = await ragResponse.json();
        setEnrichedPrompt(enrichedPrompt);
        setIsEnriching(false);
      }

      const payload = {
        conversation_id: conversationId,
        model: 'claude-3-5-sonnet-20240620',
        prompt: input,
        ...(ragEnabled && conversation.length === 0 && { enriched_prompt: enrichedPrompt }),
      };

      const promptResponse = await fetch('http://localhost:8080/talk_to_kinode:talk_to_kinode:uncentered.os/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });
      console.log('Prompt response:', promptResponse);

      if (promptResponse.ok) {
        const data = await promptResponse.json();
        const updatedConversation = data.map((content, index) => ({
          type: index % 2 === 0 ? 'question' : 'answer',
          content: content === "placeholder" ? exampleAnswer : content,
        }));
        setConversation(updatedConversation);
      } else {
        console.error('Failed to send prompt:', promptResponse.statusText);
      }
    } catch (error) {
      console.error('Error in handleSubmit:', error);
      setIsEnriching(false);
    }
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
                const [, language, code] = part.match(/```(\w+)?\n?([\s\S]+)```/) || [];
                return (
                  <SyntaxHighlighter
                    key={i}
                    language={language === 'rust' ? 'rust' : (language || 'text')}
                    style={vscDarkPlus}
                    onClick={() => copyToClipboard(code)}
                  >
                    {code.trim()}
                  </SyntaxHighlighter>
                );
              }
              return <p key={i}>{part}</p>;
            })}
          </div>
        ))}
      </div>
      <div className="enriched-prompt-container">
        {ragEnabled && (isEnriching ? (
          <p>Enriching prompt...</p>
        ) : enrichedPrompt && (
          <details>
            <summary>View Enriched Prompt</summary>
            <pre>{enrichedPrompt}</pre>
          </details>
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
            <label htmlFor="rag-checkbox">RAG (On by default)</label>
          </div>
        </form>
      </div>
    </div>
  );
};

export default ChatInterface;
