import React from "react";
import Note from "./components/Note";
import {useSetup} from "./protocol/Setup";
import {requestToIde} from "./protocol/Protocol";

function App() {
    const handleButtonClick = () => {
        requestToIde('Hello from React! send')
            .then((data) => {
                console.log("requestToIde got data : " + data);
            });
    };

    useSetup()

    return (
        <div style={{maxWidth: "100%", padding: "10px 40px"}}>
            <button onClick={handleButtonClick}>Send to IntelliJ send</button>
            <Note/>
        </div>
    );
}

export default App;

// TODO : theme, font
