
const selectedAppStorageName = "selectedApp";
const selectedFeatureStorageName = "selectedFeature";
const selectedLabelStorageName = "selectedLabel";

class Apps {

    static async init() {
        let selectedEnv = Envs.getSelected();

        if (selectedEnv) {
            await this.request(selectedEnv);
        }
    }

    static async request(env: string) {
        let header = document.getElementById("header");

        if (AppContext.apps) {
            AppContext.apps.dispose();
        }

        header.innerHTML = "Loading...";

        let data: ReleaseAppsHttpModel = await $.ajax({ url: "/api/release/all", data: { env: env } });


        if (data.ids) {

            AppContext.labels = data.labels;
            header.innerHTML = this.generateHtml(env);

            let items: SelectItemsGroup<string>[] = [];

            for (let item of data.ids) {
                items.push({ name: item.category, items: item.ids });
            }

            AppContext.apps = new SelectAdvanced(items,
                Apps.getSelectedApp(env),
                {
                    componentId: 'select-app',
                    backgroundId: 'background',
                    getItemAsString: (item: string) => item,

                    onSelect: (value: string) => {
                        Apps.saveSelectedApp(value)
                    }
                });


        }

    }

    static generateHtml(env: string): string {

        let selectedApp = this.getSelectedApp(env);

        let selectedLabel: any = this.getSelectedLabel(env);

        console.log(selectedApp);

        return HtmlUtils.render3Table("auto", "400px", "60px", () => {

            let renderer = `<div>App:</div><div id="select-app" class="form-select" data-value="${selectedApp}">${selectedApp}</div>`;
            return renderer;

            /*
            let renderer = '<span>App:</span><select id="app" class="form-select"  onchange="Apps.saveSelected()">';
            renderer += '<option value="---">---</option>';
            if (selectedApp == "*") {
                renderer += '<option value="*" selected>All</option>';
            } else {
                renderer += '<option value="*">All</option>';
            }


            for (let item of AppContext.apps) {

                renderer += `<optgroup label="${item.category}">`;

                for (let itm of item.ids) {
                    if (selectedApp == itm) {
                        renderer += '<option value="' + itm + '" selected>' + itm + "</option>";
                    } else {
                        renderer += '<option value="' + itm + '">' + itm + "</option>";
                    }
                }

                renderer += "</optgroup>";
            }



            return renderer + "</select>";
            */
        },
            () => {

                let items = [];

                items.push("---");

                for (let itm of AppContext.labels) {
                    items.push(itm);
                }

                return '<span>Label:</span>' + HtmlHelpers.renderSelect("label", "Apps.saveLabelSelected()", items, selectedLabel);

            },

            () => {
                return `<button class="btn btn-primary" onclick="Apps.onExecute()">Execute</button>`;
            }
        );


    }

    static getSelectedAppValue(): string {
        let app: any = document.getElementById("select-app");
        return app.getAttribute("data-value");
    }


    static saveSelectedApp(value: string) {
        MyStorage.saveSelectedByEnv(selectedAppStorageName, value);
    }

    static saveLabelSelected() {
        let app: any = document.getElementById("label");
        let selectedLabel: string = app.value;
        MyStorage.saveSelectedByEnv(selectedLabelStorageName, selectedLabel);
    }


    static getSelectedApp(env: string): string {
        let storageValue = MyStorage.getAsObject(selectedAppStorageName);
        let result = storageValue[env];

        if (result == '---') {
            return undefined;
        }

        return result;
    }



    static getSelectedLabel(env: string): string {
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


    static getArgToExecute(env: string): string {


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

        let data = {}

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












