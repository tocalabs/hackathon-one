import React, { useState } from 'react';
import ReactConfetti from 'react-confetti';
import Typing from 'react-typing-animation';
import { Typography } from '@mui/material';
import ChatBox from './ChatBox';
import Spacer from './Spacer';
export default function Home({logo}) {
    const [done, setDone] = useState(false);
    return (
        <>
            <img src={logo} className="App-logo" alt="logo" />
            <Typing speed={100}>
                <Typography variant="h1">
                    Hackathon
                </Typography>
            </Typing>
            <Typing startDelay={1000} onFinishedTyping={() => setDone(false)}>
                <Typography variant="subtitle1">
                    This works bro
                </Typography>
            </Typing>
            <Spacer />
            {done && <ReactConfetti width={window.width} height={window.height} />}
            <ChatBox onEnter={() => setDone(true)}/>
        </>
    );
}