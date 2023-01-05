import {hasFinderOpen, hasSettingsOpen, settings, sidebarOpen} from "../stores";

export const onKeydown = (event: KeyboardEvent) => {
    // open settings: CTRL + ,
    if (event.ctrlKey && event.key === ',') {
        event.preventDefault();
        hasSettingsOpen.set(true);
    }

    // Toggle File Tree: CTRL + SHIFT + E
    if (event.ctrlKey && event.shiftKey && event.key === 'E') {
        event.preventDefault();
        settings.update(s => {
            s.file_tree_open = !s.file_tree_open;
            return s;
        })

        sidebarOpen.update(s => !s);
    }

    // Toggle Finder: CTRL + P
    if (event.ctrlKey && event.key === 'p') {
        event.preventDefault();
        hasFinderOpen.update(s => !s);
    }
}