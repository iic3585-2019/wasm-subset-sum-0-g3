import * as wasm from "@ndleyton/subset-sum-0";

const updateRepresentation = (array, id) => {
  document.getElementById(id).innerHTML = "[" + array.toString() + "]";
};

const main = () => {
  let array = [];

  document.getElementById("addElementButton").onclick = function(e) {
    const numberInput = document.getElementById("addNumberInput");
    const rawValue = numberInput.value;
    array.push(parseInt(rawValue, 10));
    updateRepresentation(array, "array");
    numberInput.value = "";
  };

  document.getElementById("testArrayButton").onclick = function(e) {
    const res = wasm.subset_sum(array, 0);
    document.getElementById("result").innerHTML = res;
  };

  document.getElementById("clearArrayButton").onclick = function(e) {
    array = [];
    updateRepresentation(array, "array");
    document.getElementById("result").innerHTML = "...";
  };
};

main();
