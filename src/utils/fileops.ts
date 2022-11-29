import { invoke } from "@tauri-apps/api/tauri"

export const get_contents = async (fqpn: string) => {
    return await invoke("file_contents", { path: fqpn })
}

export const save_contents = async (fqpn: string, contents: string) => {
    return await invoke("write_file", { path: fqpn, content: contents })
}