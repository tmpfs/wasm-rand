if (window.Worker) {
  // Worker communication.
  const worker = new Worker(new URL("./worker.js", import.meta.url));
  worker.onmessage = (e) => {
    const {values} = e.data;
    console.log('Javascript got random value', values);
    const list = document.querySelector('.value');
    values.forEach((num) => {
      const el = document.createElement('li');
      el.innerText = num.toString();
      list.appendChild(el);
    })
  }
} else {
  console.log('Your browser doesn\'t support web workers.');
}
