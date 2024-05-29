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
    static render4Table(widths, p1, p2, p3, p4) {
        let result = `<table style="width:100%"><tr><td style="width:${widths[0]}">`;
        result += p1();
        result += `</td><td style="width:${widths[1]}">`;
        result += p2();
        result += `</td><td style="width:${widths[2]}">`;
        result += p3();
        result += `</td><td style="width:${widths[3]}">`;
        result += p4();
        return result + `</td></tr></table>`;
    }
}

// html.js
class HtmlHelpers {
    static renderSelect(id, callback, options, selected) {
        let renderer = `<select id="${id}" class="form-select"  onchange="${callback}">`;
        for (let item of options) {
            if (selected == item) {
                renderer += '<option value="' + item + '" selected>' + item + "</option>";
            }
            else {
                renderer += '<option value="' + item + '">' + item + "</option>";
            }
        }
        return renderer + "</select>";
    }
}

// storage.js
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
            let doc = document.getElementById("content");
            doc.innerHTML = data.html;
            doc.scrollTo(0, doc.scrollHeight);
            if (data.finished == true) {
                AppContext.selectedProcess = undefined;
            }
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
            let featureBadge = "";
            if (itm.feature) {
                featureBadge = '<div><span class="badge text-bg-primary">' + itm.feature + '</span></div>';
            }
            if (itm.id == selected) {
                result += `<div class="btn btn-secondary" style="width:100%">${itm.id}${featureBadge}</div>`;
            }
            else {
                result += `<div data-id="${itm.id}" class="btn btn-outline-secondary" style="width:100%" onclick="Envs.select(this)">${itm.id}${featureBadge}</div>`;
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
            AppContext.apps = [];
            header.innerHTML = "Loading...";
            let data = yield $.ajax({ url: "/api/release/all", data: { env: env } });
            if (data.ids) {
                AppContext.apps = data.ids;
                AppContext.labels = data.labels;
                AppContext.features = data.features;
                header.innerHTML = this.generateHtml(env);
            }
        });
    }
    static generateHtml(env) {
        let selectedApp = this.getSelectedApp(env);
        let selectedLabel = this.getSelectedLabel(env);
        console.log(selectedApp);
        return HtmlUtils.render3Table("auto", "400px", "60px", () => {
            let renderer = '<span>App:</span><select id="app" class="form-select"  onchange="Apps.saveSelected()">';
            renderer += '<option value="---">---</option>';
            if (selectedApp == "*") {
                renderer += '<option value="*" selected>All</option>';
            }
            else {
                renderer += '<option value="*">All</option>';
            }
            for (let item of AppContext.apps) {
                renderer += `<optgroup label="${item.category}">`;
                for (let itm of item.ids) {
                    if (selectedApp == itm) {
                        renderer += '<option value="' + itm + '" selected>' + itm + "</option>";
                    }
                    else {
                        renderer += '<option value="' + itm + '">' + itm + "</option>";
                    }
                }
                renderer += "</optgroup>";
            }
            return renderer + "</select>";
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
    static saveSelected() {
        let app = document.getElementById("app");
        let selectedApp = app.value;
        MyStorage.saveSelectedByEnv(selectedAppStorageName, selectedApp);
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

