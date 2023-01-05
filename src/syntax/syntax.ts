export function doInBetweenReplace(regex: RegExp, elementId: string, specialChars: string, lang: string, lineno: number, caretLine: number, line: string): string {
    if (lineno != caretLine) {
        line = line.replace(regex, `<span class="syntax ${lang} ${elementId}">$1</span>`)
    } else {
        line = line.replace(regex, `<span class="syntax ${lang} ${elementId} target">${specialChars}</span>`)
    }
    
    return line
}