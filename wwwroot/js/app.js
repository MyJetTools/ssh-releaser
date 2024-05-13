// utils.js
class HtmlUtils {
    static renderSplitTable(leftWidth, rightWidth, leftPanel, rightPanel) {
        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;
        result += leftPanel();
        result += `</td><td style="width:${rightWidth}">`;
        result += rightPanel();
        return result + `</td></tr></table>`;
    }
    static render3Table(leftWidth, middleWidth, rightWidth, leftPanel, middlePanel, rightPanel) {
        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;
        result += leftPanel();
        result += `</td><td style="width:${middleWidth}">`;
        result += middlePanel();
        result += `</td><td style="width:${rightWidth}">`;
        result += rightPanel();
        return result + `</td></tr></table>`;
    }
}

// app.js
class AppContext {
}
setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        Envs.refresh();
        Apps.init();
    });
}, 100);
setInterval(function () {
    if (AppContext.selectedProcess) {
        $.ajax({ url: "/api/release/logs", data: { id: AppContext.selectedProcess } }).then(function (data) {
            document.getElementById("content").innerHTML = data;
        });
    }
}, 1000);

// envs.js
class Envs {
    static refresh() {
        let html = this.render(this.getSelected());
        document.getElementById("left-panel").innerHTML = html;
    }
    static render(selected) {
        let result = "";
        for (let itm of AppContext.envs) {
            if (itm == selected) {
                result += '<div class="btn btn-secondary" style="width:100%">' + itm + "</div>";
            }
            else {
                result += '<div data-id="' + itm + '" class="btn btn-outline-secondary" style="width:100%" onclick="Envs.select(this)">' + itm + "</div>";
            }
        }
        return result;
    }
    static getSelected() {
        return localStorage.getItem("selectedEnv");
    }
    static select(itm) {
        let id = itm.getAttribute("data-id");
        localStorage.setItem("selectedEnv", id);
        this.refresh();
        Apps.request(id);
    }
}

// apps.js
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
            AppContext.apps = [];
            header.innerHTML = "Loading...";
            let data = yield $.ajax({ url: "/api/release/all", data: { env: env } });
            if (data.ids) {
                AppContext.apps = data.ids;
                AppContext.labels = data.labels;
                header.innerHTML = this.generateHtml(env);
            }
        });
    }
    static generateHtml(env) {
        let selectedApp = this.getSelectedApp(env);
        let selectedLabels = this.getSelectedLabels(env);
        console.log(selectedApp);
        return HtmlUtils.render3Table("300px", "auto", "60px", () => {
            let renderer = '<select id="app" class="form-select"  onchange="Apps.saveSelected()">';
            renderer += '<option value="---">---</option>';
            if (selectedApp == "*") {
                renderer += '<option value="*" selected>All</option>';
            }
            else {
                renderer += '<option value="*">All</option>';
            }
            for (let itm of AppContext.apps) {
                if (selectedApp == itm) {
                    renderer += '<option value="' + itm + '" selected>' + itm + "</option>";
                }
                else {
                    renderer += '<option value="' + itm + '">' + itm + "</option>";
                }
            }
            return renderer + "</select>";
        }, () => {
            let rendered = "";
            for (let itm of AppContext.labels) {
                let myClass = "text-bg-light";
                if (selectedLabels.includes(itm)) {
                    myClass = "text-bg-dark";
                }
                rendered += `<span data-badge="${itm}" class="badge ${myClass}" style="cursor:pointer" onclick="Apps.onBadgeClick(this)">${itm}</span>`;
            }
            return rendered;
        }, () => {
            return `<button class="btn btn-primary" onclick="Apps.onExecute()">Execute</button>`;
        });
    }
    static saveSelected() {
        let app = document.getElementById("app");
        let selectedApp = app.value;
        this.saveSelectedApp(selectedApp);
    }
    static saveSelectedApp(selectedApp) {
        let storageValue = this.getFromStorageAsObject(selectedAppStorageName);
        let env = Envs.getSelected();
        storageValue[env] = selectedApp;
        localStorage.setItem(selectedAppStorageName, JSON.stringify(storageValue));
    }
    static getSelectedApp(env) {
        let storageValue = this.getFromStorageAsObject(selectedAppStorageName);
        let result = storageValue[env];
        if (result == '---') {
            return undefined;
        }
        return result;
    }
    static getFromStorageAsObject(name) {
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
    static getSelectedLabels(env) {
        let valueFromStorage = this.getFromStorageAsObject("selectedLabels");
        let badges = valueFromStorage[env];
        if (!badges) {
            return [];
        }
        else {
            return JSON.parse(badges);
        }
    }
    static onBadgeClick(itm) {
        let env = Envs.getSelected();
        let badge = itm.getAttribute("data-badge");
        console.log(itm);
        let valueFromStorage = this.getFromStorageAsObject("selectedLabels");
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
    static getSelectedToExecute(env) {
        let result = [];
        let app = this.getSelectedApp(env);
        if (app) {
            result.push(app);
        }
        for (let label of this.getSelectedLabels(env)) {
            result.push(label);
        }
        return result;
    }
    static onExecute() {
        let env = Envs.getSelected();
        let args = this.getSelectedToExecute(env);
        console.log(env);
        console.log(args);
        $.ajax({ method: "POST", url: "/api/release/execute", data: { env: env, arg: args[0] } }).then(function (data) {
            console.log(data);
            AppContext.selectedProcess = data;
        });
    }
}

