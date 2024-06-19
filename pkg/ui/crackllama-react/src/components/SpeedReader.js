import React, { useState, useEffect } from 'react';
import axios from 'axios';

function SpeedReader() {
    const [promptText, setPromptText] = useState('');
    const [wpm, setWpm] = useState(600);
    const [display, setDisplay] = useState('');
    const [speedReadingActive, setSpeedReadingActive] = useState(false);
    const [isPaused, setIsPaused] = useState(false);
    const [words, setWords] = useState([]);
    const [index, setIndex] = useState(0);

    useEffect(() => {
        document.addEventListener('keydown', handleGlobalKeyPress);
        return () => {
            document.removeEventListener('keydown', handleGlobalKeyPress);
        };
    }, [speedReadingActive, isPaused]);

    const handleKeyPress = (event) => {
        if (event.key === "Enter" && !event.shiftKey) {
            event.preventDefault();
            submitPrompt();
        }
    };

    const submitPrompt = () => {
        axios.post('http://localhost:8080/crackllama:crackllama:template.os/prompt', { prompt: promptText })
            .then(response => {
                speedRead(response.data);
                setPromptText('');
            })
            .catch(error => {
                console.error('Error:', error);
                setDisplay('Error fetching response');
            });
    };

    const speedRead = (text) => {
        // Implement the speed reading logic here
    };

    const handleGlobalKeyPress = (event) => {
        if (event.key === "Escape" && speedReadingActive) {
            finishSpeedRead();
        } else if (event.key === " " && speedReadingActive) {
            event.preventDefault();
            pauseContinueSpeedRead();
        }
    };

    const pauseContinueSpeedRead = () => {
        if (speedReadingActive) {
            if (isPaused) {
                // Continue speed reading
                setIsPaused(false);
            } else {
                // Pause speed reading
                setIsPaused(true);
            }
        }
    };

    const finishSpeedRead = () => {
        // Finish speed reading
        setSpeedReadingActive(false);
    };

    return (
        <div>
            <textarea
                value={promptText}
                onChange={(e) => setPromptText(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Enter your prompt"
            />
            <div>
                <button onClick={() => setWpm(wpm - 20)}>&lt;</button>
                <span>{wpm} WPM</span>
                <button onClick={() => setWpm(wpm + 20)}>&gt;</button>
            </div>
            <div>{display}</div>
        </div>
    );
}

export default SpeedReader;