import React from "react";
import Note from "./components/Note";
import {sendToIde} from "./protocol/Protocol";

function App() {
    const handleButtonClick = () => {
        sendToIde('Hello from React! send');
    };

    return (
        <div style={{maxWidth: "100%", padding: "10px 40px"}}>
            <button onClick={handleButtonClick}>Send to IntelliJ send</button>
            <Note/>
        </div>
    );
}

export default App;
