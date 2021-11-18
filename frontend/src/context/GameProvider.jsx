import React, { createContext, useContext, useMemo, useState } from "react";

const GameContext = createContext();

export default function GameProvider({ children }) {
    const [gameState, setGameState] = useState({});
    const value = useMemo(() => ({gameState, setGameState}), [gameState]);
  return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
}

export function useGameProvider() {
    const context = useContext(GameContext);
    if (!context) {
        throw new Error('useGameProvider is not inside of GameProvider');
    }
    return context;
}