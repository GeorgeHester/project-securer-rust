import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
//import * as  { appDataDir } from '@tauri-apps/api/path';
import * as tauri from "@tauri-apps/api";

interface Props
{

};

export const LoginForm: React.FC<Props> = () =>
{
    let [output, setOutput] = useState('');

    useEffect(() =>
    {
        async function callCommand()
        {
            const appDataDirectory: string = await tauri.path.appDataDir();
            



            let commandOutput: string = await invoke("sha_512", { input: "Testing" });
            let commandOutput2: string = await invoke("create_vault", { user_id: "1", vault_id: "2" });
            console.log(commandOutput2);
            setOutput(commandOutput);
        };

        callCommand();
    }, []);

    return (
        <h1>{output}</h1>
    );
};
