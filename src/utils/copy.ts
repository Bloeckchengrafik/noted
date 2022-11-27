export function copy(text: string) {
    const textarea = document.createElement("textarea");
    textarea.textContent = text;
    textarea.style.position = "fixed";
    textarea.style.width = '2em';
    textarea.style.height = '2em';
    textarea.style.padding = "0";
    textarea.style.border = 'none';
    textarea.style.outline = 'none';
    textarea.style.boxShadow = 'none';
    textarea.style.background = 'transparent';
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    try {
        document.execCommand("copy");
        document.body.removeChild(textarea);
    } catch (e) {
        document.body.removeChild(textarea);
    }

}