import { HashRouter } from "react-router-dom";
import "./App.css";
import { Home } from "./pages/Home/Home";
import { WebSocketProvider } from "./contexts/webSocketContext";

function App() {
  return (
    <WebSocketProvider>
      <HashRouter>
        <Home />
      </HashRouter>
    </WebSocketProvider>
  );
}

export default App;
