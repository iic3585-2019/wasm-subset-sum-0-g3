import * as wasm from "@ndleyton/subset-sum-0";

const updateRepresentation = (array, id) => {
  document.getElementById(id).innerHTML = "[" + array.toString() + "]";
};

const main = () => {
  let array = [];

  document.getElementById("addElementButton").onclick = function(e) {
    const numberInput = document.getElementById("addNumberInput");
    const rawValues = numberInput.value.split(",");
    array = [...array, ...rawValues.map(x => parseInt(x, 10))];
    updateRepresentation(array, "array");
    numberInput.value = "";
  };

  document.getElementById("testArrayButton").onclick = function(e) {
    const t0 = performance.now();
    const res = wasm.subset_sum(array, 0);
    var t1 = performance.now();
    document.getElementById("result").innerHTML =
      res + " in " + (t1 - t0) + " ms.";
  };

  document.getElementById("clearArrayButton").onclick = function(e) {
    array = [];
    updateRepresentation(array, "array");
    document.getElementById("result").innerHTML = "...";
  };
};

main();
