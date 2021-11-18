import { Paper, TextField } from '@mui/material';
import React, { useCallback, useState } from 'react';
import { styled } from '@mui/material/styles';

const StyledPaper = styled(Paper)(({theme}) => ({
    padding: theme.spacing(2)
}));

export default function ChatBox({onEnter = () => {}}) {
    const [message, setMessage] = useState('');
    const keyPress = useCallback(evt => {
        if (evt.keyCode === 13) {
            onEnter();
        }
    }, [onEnter]);
    return (
        <StyledPaper elavation={10}>
            <TextField
                value={message}
                label="Name Your Character"
                variant="outlined"
                fullWidth
                onKeyDown={keyPress}
                onChange={evt => setMessage(evt.target.value)}
            />
        </StyledPaper>
    )
}