import React from 'react';
import { styled } from '@mui/material/styles';

const Styled = styled(React.Fragment)(({theme}) => ({
    padding: theme.spacing(1)
}));

export default function Spacer() {
    return <Styled />
}