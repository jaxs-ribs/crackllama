import React, { useState, useEffect } from 'react';

const ProgressCounter = ({ isLoading }) => {
  const [count, setCount] = useState(0);

  useEffect(() => {
    let interval;
    if (isLoading) {
      interval = setInterval(() => {
        setCount((prevCount) => (prevCount + 1) % 100);
      }, 50);
    } else {
      setCount(0);
    }
    return () => clearInterval(interval);
  }, [isLoading]);

  return (
    <div className="progress-counter">
      <div className="progress-bar" style={{ width: `${count}%` }}></div>
      <span className="progress-text">{count}%</span>
    </div>
  );
};

export default ProgressCounter;