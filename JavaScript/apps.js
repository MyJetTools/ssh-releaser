var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const selectedAppStorageName = "selectedApp";
const selectedFeatureStorageName = "selectedFeature";
const selectedLabelStorageName = "selectedLabel";
class Apps {
    static init() {
        return __awaiter(this, void 0, void 0, function* () {
            let selectedEnv = Envs.getSelected();
            if (selectedEnv) {
                yield this.request(selectedEnv);
            }
        });
    }
    static request(env) {
        return __awaiter(this, void 0, void 0, function* () {
            let header = document.getElementById("header");
            if (AppContext.apps) {
                AppContext.apps.dispose();
            }
            header.innerHTML = "Loading...";
            let data = yield $.ajax({ url: "/api/release/all", data: { env: env } });
            if (data.ids) {
                AppContext.labels = data.labels;
                header.innerHTML = this.generateHtml(env);
                let items = [];
                for (let item of data.ids) {
                    items.push({ name: item.category, items: item.ids });
                }
                AppContext.apps = new SelectAdvanced(items, Apps.getSelectedApp(env), {
                    componentId: 'select-app',
                    backgroundId: 'background',
                    getItemValue: (item) => item.id,
                    getItemAsHtml: (item) => {
                        let result = item.id;
                        for (let feature of item.exclude_features) {
                            result += `<span class="badge text-bg-danger"><s>` + feature + `</s></span>`;
                        }
                        console.log(result);
                        return result;
                    },
                    onSelect: (value) => {
                        Apps.saveSelectedApp(value);
                    }
                });
            }
        });
    }
    static generateHtml(env) {
        let selectedApp = this.getSelectedApp(env);
        let selectedLabel = this.getSelectedLabel(env);
        console.log(selectedApp);
        return HtmlUtils.render3Table("auto", "400px", "60px", () => {
            let renderer = `<div>App:</div><div id="select-app" class="form-select" data-value="${selectedApp}">${selectedApp}</div>`;
            return renderer;
        }, () => {
            let items = [];
            items.push("---");
            for (let itm of AppContext.labels) {
                items.push(itm);
            }
            return '<span>Label:</span>' + HtmlHelpers.renderSelect("label", "Apps.saveLabelSelected()", items, selectedLabel);
        }, () => {
            return `<button class="btn btn-primary" onclick="Apps.onExecute()">Execute</button>`;
        });
    }
    static getSelectedAppValue() {
        let app = document.getElementById("select-app");
        return app.getAttribute("data-value");
    }
    static saveSelectedApp(value) {
        MyStorage.saveSelectedByEnv(selectedAppStorageName, value);
    }
    static saveLabelSelected() {
        let app = document.getElementById("label");
        let selectedLabel = app.value;
        MyStorage.saveSelectedByEnv(selectedLabelStorageName, selectedLabel);
    }
    static getSelectedApp(env) {
        let storageValue = MyStorage.getAsObject(selectedAppStorageName);
        let result = storageValue[env];
        if (result == '---') {
            return undefined;
        }
        return result;
    }
    static getSelectedLabel(env) {
        let storageValue = MyStorage.getAsObject(selectedLabelStorageName);
        let result = storageValue[env];
        if (result == '---') {
            return undefined;
        }
        return result;
    }
    static onBadgeClick(itm) {
        let env = Envs.getSelected();
        let badge = itm.getAttribute("data-badge");
        console.log(itm);
        let valueFromStorage = MyStorage.getAsObject("selectedLabels");
        let badges = valueFromStorage[env];
        let badgesAsArray;
        if (!badges) {
            badgesAsArray = [];
        }
        else {
            badgesAsArray = JSON.parse(badges);
        }
        if (badgesAsArray.includes(badge)) {
            badgesAsArray = badgesAsArray.filter((b) => b != badge);
        }
        else {
            badgesAsArray.push(badge);
        }
        valueFromStorage[env] = JSON.stringify(badgesAsArray);
        localStorage.setItem("selectedLabels", JSON.stringify(valueFromStorage));
        let header = document.getElementById("header");
        header.innerHTML = this.generateHtml(env);
    }
    static getArgToExecute(env) {
        let app = this.getSelectedApp(env);
        if (app) {
            return app;
        }
        let label = this.getSelectedLabel(env);
        if (label) {
            return label;
        }
        return undefined;
    }
    static onExecute() {
        let env = Envs.getSelected();
        let arg = this.getArgToExecute(env);
        document.getElementById("content").innerHTML = "Starting Script...";
        let data = {};
        data["env"] = env;
        data["arg"] = arg;
        $.ajax({ method: "POST", url: "/api/release/execute", data: data }).then(function (result) {
            console.log(result);
            AppContext.selectedProcess = result;
        }).fail(function (result) {
            document.getElementById("content").innerHTML = result;
        });
    }
}
//# sourceMappingURL=apps.js.map