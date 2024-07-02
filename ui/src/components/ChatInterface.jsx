import React, { useState, useRef, useEffect, useCallback } from 'react';
import './ChatInterface.css';

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
    
    const exampleAnswer = `Here's an example of how to create a simple React component:

To create a basic React component, you can follow these steps:

1. First, import React:

\`\`\`jsx
import React from 'react';
\`\`\`

2. Then, define your component as a function:

\`\`\`jsx
function MyComponent() {
  return (
    <div>
      <h1>Hello, World!</h1>
      <p>This is my first React component.</p>
    </div>
  );
}
\`\`\`

3. Finally, export your component:

\`\`\`jsx
export default MyComponent;
\`\`\`

You can then use this component in other parts of your application by importing it and rendering it like this:

\`\`\`jsx
import MyComponent from './MyComponent';

function App() {
  return (
    <div>
      <MyComponent />
    </div>
  );
}
\`\`\`

This is a basic example, but it demonstrates the fundamental structure of a React component.`;

    const newAnswer = { type: 'answer', content: exampleAnswer };
    setConversation(prev => [...prev, newAnswer]);
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
