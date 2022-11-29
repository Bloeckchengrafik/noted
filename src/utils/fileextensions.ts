export type ExtensionType = 'text' | 'md' | 'image'

export const get_type = (fqpn: string): ExtensionType => {
    const ext = (fqpn.split(".").pop()!).toLowerCase()
    if (ext === 'md') {
        return 'text'
    }
    if (['png', 'jpg', 'jpeg', 'gif'].includes(ext)) {
        return 'image'
    }

    return 'text'
}