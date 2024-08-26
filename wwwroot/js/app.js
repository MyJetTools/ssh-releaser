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

// select_advanced.js
class SelectAdvanced {
    constructor(items, selected, setup) {
        this.items = items;
        this.htmlComponent = document.getElementById(setup.componentId);
        this.backgroundElement = document.getElementById(setup.backgroundId);
        this.getItemAsHtml = setup.getItemAsHtml;
        this.onSelect = setup.onSelect;
        this.getItemValue = setup.getItemValue;
        SelectAdvanced.enteredFilter = "";
        SelectAdvanced.current = this;
        this.selected = selected;
        this.htmlComponent.addEventListener("click", () => {
            this.backgroundElement.classList.remove("hidden");
            this.backgroundElement.classList.add("visible");
            this.showContent();
        });
    }
    dispose() {
    }
    iterateItems(callback) {
        for (let group of this.items) {
            for (let item of group.items) {
                callback(item);
            }
        }
    }
    renderThisItem(item) {
        let itemAsString = this.getItemValue(item).toLowerCase();
        return itemAsString.includes(SelectAdvanced.enteredFilter);
    }
    groupHasContentToShow(group) {
        if (SelectAdvanced.enteredFilter == "") {
            console.log("selected is empty");
            return true;
        }
        for (let item of group.items) {
            if (this.renderThisItem(item)) {
                return true;
            }
        }
        console.log("exiting at end");
        return false;
    }
    static onFilterChange(aThis) {
        SelectAdvanced.enteredFilter = aThis.value.toLowerCase();
        console.log("Filter changed: " + SelectAdvanced.enteredFilter);
        SelectAdvanced.current.reRenderItems();
    }
    showContent() {
        let width = this.htmlComponent.clientWidth;
        let result = `<div class="select-popup" style="width:${width}px"  onclick="window.event.cancelBubble = true;"><div class="select-filter-panel" style="width:${width}px"><input class="select-filter" oninput="SelectAdvanced.onFilterChange(this)"/></div>`;
        result += `<div id="select-items">` + this.renderItems() + `</div>`;
        result += `</div>`;
        this.backgroundElement.innerHTML = result;
    }
    reRenderItems() {
        let items = document.getElementById("select-items");
        items.innerHTML = this.renderItems();
    }
    renderItems() {
        let result = "";
        for (let group of this.items) {
            if (!this.groupHasContentToShow(group)) {
                continue;
            }
            result += `<div class="select-group disable-selection">${group.name}</div>`;
            for (let item of group.items) {
                if (this.renderThisItem(item)) {
                    let selectedAttr = "";
                    if (this.selected == this.getItemValue(item)) {
                        selectedAttr = "select-item-selected";
                    }
                    let content = this.getItemAsHtml(item);
                    let value = this.getItemValue(item);
                    result += `<div data-value="${value}" class="select-item disable-selection ${selectedAttr}" onclick="SelectAdvanced.onClick(this)">${content}</div>`;
                }
            }
        }
        return result;
    }
    static onClick(aThis) {
        let value = aThis.getAttribute("data-value");
        this.current.selectItem(value);
    }
    selectItem(value) {
        SelectAdvanced.enteredFilter = "";
        this.selected = value;
        this.htmlComponent.innerHTML = value;
        this.htmlComponent.setAttribute("data-value", value);
        AppContext.hideBackground();
        this.onSelect(value);
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
    static renderProducts(products) {
        let selectedProduct = AppContext.getSelectedProduct();
        let result = `<select class="form-select" style="width:100%" onchange="AppContext.onProductSelect(this)">`;
        for (let product of products) {
            if (product == selectedProduct) {
                result += `<option selected>${product}</option>`;
            }
            else {
                result += `<option>${product}</option>`;
            }
        }
        return result + `</select>`;
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
    static hideBackground() {
        let background = document.getElementById("background");
        background.classList.remove("visible");
        background.classList.add("hidden");
    }
    static getProducts() {
        return Object.keys(this.envs);
    }
    static getEnvs(product) {
        return this.envs[product];
    }
    static getSelectedProduct() {
        let result = localStorage.getItem("selectedProduct");
        if (!result) {
            result = this.getProducts()[0];
            localStorage.setItem("selectedProduct", result);
        }
        return result;
    }
    static onProductSelect(itm) {
        let value = itm.value;
        console.log(value);
        localStorage.setItem("selectedProduct", value);
        Envs.refresh();
    }
}
setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        let products = AppContext.getProducts();
        let productsSelect = HtmlHelpers.renderProducts(products);
        document.getElementById("product-select-panel").innerHTML = productsSelect;
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
function backgroundClick() {
    let background = document.getElementById("background");
    background.classList.remove("visible");
    background.classList.add("hidden");
}

// envs.js
class Envs {
    static refresh() {
        let html = this.render(this.getSelected());
        document.getElementById("env-select-panel").innerHTML = html;
    }
    static render(selected) {
        let result = "";
        let selectedProduct = AppContext.getSelectedProduct();
        for (let itm of AppContext.getEnvs(selectedProduct)) {
            let featureBadge = "";
            if (itm.features) {
                let odd = false;
                for (let feature of itm.features) {
                    let badgeType = getBadgeType(odd);
                    featureBadge += '<div><span class="badge ' + badgeType + '">' + feature + '</span></div>';
                    odd = !odd;
                }
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
function getBadgeType(odd) {
    if (odd) {
        return "text-bg-warning";
    }
    return "text-bg-primary";
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
            if (AppContext.apps) {
                AppContext.apps.dispose();
            }
            header.innerHTML = "Loading...";
            let product = AppContext.getSelectedProduct();
            let data = yield $.ajax({ url: "/api/release/all", data: { env: env, product: product } });
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
                        for (let feature of item.include_features) {
                            result += `<span class="badge text-bg-success">` + feature + `</span>`;
                        }
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
        data["product"] = AppContext.getSelectedProduct();
        $.ajax({ method: "POST", url: "/api/release/execute", data: data }).then(function (result) {
            console.log(result);
            AppContext.selectedProcess = result;
        }).fail(function (result) {
            document.getElementById("content").innerHTML = result;
        });
    }
}

