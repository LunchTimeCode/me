

export function chooseTheme(themeInput){
  localStorage.setItem("theme", themeInput);
}

htmx.on("htmx:configRequest", (e)=> {
    e.detail.headers["X-Theme"] =  localStorage.getItem("theme");
})