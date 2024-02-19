const { invoke } = window.__TAURI__.core;


function greet() {
  invoke("ok").then((err) => {
    document.querySelector("#ok-msg").textContent = `type of ok: ${typeof err}, ok: ${err}`
  })
  invoke("err").catch((err) => {
    document.querySelector("#err-msg").textContent = `type of err: ${typeof err}, err: ${err}`
  })
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
