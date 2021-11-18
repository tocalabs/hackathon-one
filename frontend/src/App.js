import logo from './toca_logo_solid_pinkandwhite.svg';
import './App.css';
import GameProvider from './context/GameProvider';
import Home from './components/Home';

function App() {
  return (
    <GameProvider>
      <Home />
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Hackathon
          </p>
          <div className="App-Subtitle">
            This works bro
          </div>
        </header>
      </div>
    </GameProvider>
  );
}

export default App;
