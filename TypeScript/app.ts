
class AppContext {
    static envs: string[];
    static apps: string[];
    static labels: string[];

    static selectedProcess: string;
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



