import Worker from 'worker-loader!./worker.js';

if (window.Worker) {
  // Worker communication.
  const worker = new Worker('worker.js');
  worker.onmessage = (e) => {
    const {value} = e.data;
    console.log('Javascript got random value', value);
    const text = document.querySelector('.value span');
    text.innerText = value;
  }
} else {
  console.log('Your browser doesn\'t support web workers.');
}
