import {settings} from "../stores";

export const changeSizeKeybindListener = (event: KeyboardEvent) => {
    // CTRL + + or CTRL + -
    if (event.ctrlKey && (event.key === '+' || event.key === '-')) {
        event.preventDefault();
        const size = event.key === '+' ? 1 : -1;
        settings.update(s => {
            s.editor_font_size += size;
            return s;
        })
    }
}