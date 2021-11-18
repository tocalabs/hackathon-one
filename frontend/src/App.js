import React from 'react';
import './App.css';
import GameProvider from './context/GameProvider';
import Home from './components/Home';
import logo from './toca_logo_solid_pinkandwhite.svg';
import { createTheme, ThemeProvider } from '@mui/material/styles';

const theme = createTheme({
  typography: {
    fontFamily: 'Press Start 2P'
  }
})

function App() {
  return (
    <ThemeProvider theme={theme}>
      <GameProvider>
        <Home logo={logo} />
      </GameProvider>
    </ThemeProvider>
  );
}

export default App;
