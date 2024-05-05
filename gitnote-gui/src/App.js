import React, {useEffect, useState} from "react";
import Note from "./components/Note";
import {requestToIde} from "./protocol/Protocol";
import {ConfigProvider, Space} from "antd";

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
        <ConfigProvider>
            <Space direction="vertical">
                <Note theme={theme}/>
            </Space>
        </ConfigProvider>

    );
}

export default App;

// TODO : theme, font
