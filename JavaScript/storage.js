class MyStorage {
    static saveSelectedByEnv(storageName, selectedApp) {
        let storageValue = this.getAsObject(storageName);
        let env = Envs.getSelected();
        storageValue[env] = selectedApp;
        localStorage.setItem(storageName, JSON.stringify(storageValue));
    }
    static getAsObject(name) {
        let storageValue = localStorage.getItem(name);
        if (!storageValue) {
            return {};
        }
        try {
            return JSON.parse(storageValue);
        }
        catch (e) {
            return {};
        }
    }
}
//# sourceMappingURL=storage.js.map