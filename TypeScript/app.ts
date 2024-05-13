
class AppContext {
    static envs: string[];
    static apps: string[];
    static labels: string[];
}


setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        Envs.refresh();
        Apps.init();
    });
}, 100);



