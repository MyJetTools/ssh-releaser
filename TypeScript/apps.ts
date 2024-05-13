
const selectedAppStorageName = "selectedApp";

class Apps {

    static async init() {
        let selectedEnv = Envs.getSelected();

        if (selectedEnv) {
            await this.request(selectedEnv);
        }
    }

    static async request(env: string) {
        let header = document.getElementById("header");
        AppContext.apps = [];
        header.innerHTML = "Loading...";

        let data = await $.ajax({ url: "/api/release/all", data: { env: env } });


        if (data.ids) {
            AppContext.apps = data.ids;
            AppContext.labels = data.labels;
            header.innerHTML = this.generateHtml(env);
        }


    }

    static generateHtml(env: string): string {

        let selectedApp = this.getSelectedApp(env);

        let selectedLabels: any = this.getSelectedLabels(env);

        console.log(selectedApp);


        return HtmlUtils.render3Table("300px", "auto", "60px", () => {

            let renderer = '<select id="app" class="form-select"  onchange="Apps.saveSelected()">';
            renderer += '<option value="---">---</option>';
            if (selectedApp == "*") {
                renderer += '<option value="*" selected>All</option>';
            } else {
                renderer += '<option value="*">All</option>';
            }
            for (let itm of AppContext.apps) {
                if (selectedApp == itm) {
                    renderer += '<option value="' + itm + '" selected>' + itm + "</option>";
                } else {
                    renderer += '<option value="' + itm + '">' + itm + "</option>";
                }
            }

            return renderer + "</select>";
        },
            () => {
                let rendered = "";

                for (let itm of AppContext.labels) {

                    let myClass = "text-bg-light";

                    if (selectedLabels.includes(itm)) {
                        myClass = "text-bg-dark";
                    }


                    rendered += `<span data-badge="${itm}" class="badge ${myClass}" style="cursor:pointer" onclick="Apps.onBadgeClick(this)">${itm}</span>`;
                }

                return rendered;
            },

            () => {
                return `<button class="btn btn-primary" onclick="Apps.onExecute()">Execute</button>`;
            }
        );


    }


    static saveSelected() {
        let app: any = document.getElementById("app");
        let selectedApp: string = app.value;
        this.saveSelectedApp(selectedApp);
    }

    static saveSelectedApp(selectedApp: string) {

        let storageValue = this.getFromStorageAsObject(selectedAppStorageName);

        let env = Envs.getSelected();

        storageValue[env] = selectedApp;

        localStorage.setItem(selectedAppStorageName, JSON.stringify(storageValue));

    }

    static getSelectedApp(env: string): string {
        let storageValue = this.getFromStorageAsObject(selectedAppStorageName);
        let result = storageValue[env];

        if (result == '---') {
            return undefined;
        }

        return result;
    }

    static getFromStorageAsObject(name: string): {} {
        let storageValue = localStorage.getItem(name);

        if (!storageValue) {
            return {};
        }

        try {
            return JSON.parse(storageValue);
        } catch (e) {
            return {};
        }
    }


    static getSelectedLabels(env: string): string[] {

        let valueFromStorage = this.getFromStorageAsObject("selectedLabels");

        let badges = valueFromStorage[env];

        if (!badges) {
            return [];
        } else {
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
        } else {
            badgesAsArray = JSON.parse(badges);
        }

        if (badgesAsArray.includes(badge)) {
            badgesAsArray = badgesAsArray.filter((b: string) => b != badge);
        }
        else {
            badgesAsArray.push(badge);
        }


        valueFromStorage[env] = JSON.stringify(badgesAsArray);

        localStorage.setItem("selectedLabels", JSON.stringify(valueFromStorage));

        let header = document.getElementById("header");
        header.innerHTML = this.generateHtml(env);
    }


    static getSelectedToExecute(env: string): string[] {

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












