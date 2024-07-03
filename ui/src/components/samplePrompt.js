export const exampleAnswer = `Here's an example of how to create a simple React component:

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