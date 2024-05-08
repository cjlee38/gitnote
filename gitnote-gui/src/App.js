import React, {useEffect, useState} from "react";
import Note from "./components/Note";
import {requestToIde} from "./protocol/Protocol";
import {Space} from "antd";

function App() {
    const [theme, setTheme] = useState({
        editorBackground: '#333333',
        text: "#dddddd"
    });

    // useSetup()
    useEffect(() => {
        requestToIde("theme", {})
            .then((data) => {
                console.log("Theme data:", JSON.stringify(data));
                setTheme({
                    background: intToHexColor(data['background']),
                    editorBackground: intToHexColor(data['editorBackground']),
                    text: intToHexColor(data['text'])
                });
            })
            .catch((error) => {
                console.log("Error fetching theme:", error);
            });
    }, []);

    useEffect(() => {
        document.body.style.backgroundColor = theme.background;
    }, [theme]);

    const intToHexColor = (colorInt) => {
        // Assuming colorInt is in the format 0xAARRGGBB
        let color = (colorInt & 0x00FFFFFF).toString(16).toUpperCase();
        return `#${'000000'.substring(0, 6 - color.length) + color}`;
    };

    return (
        <Space
            direction="vertical"
            align="center"
            style={{width: '100%', justifyContent: 'center'}}
        >
            <Note theme={theme}/>
        </Space>
    );
}

export default App;

// TODO : theme, font
