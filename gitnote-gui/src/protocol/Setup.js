import {useEffect} from "react";
import {requestToIde} from "./Protocol";

function setUpTheme() {
    requestToIde("theme", {})
        .then((data) => {
            console.log("theme data = " + JSON.stringify(data));

        }).catch((error) => {
            console.log("requestToIde got error : " + error);
        }
    );
}

export const useSetup = () => {
    useEffect(() => {
        setUpTheme();
    }, []);
    // TODO : load configuration for theme & fonts
}

