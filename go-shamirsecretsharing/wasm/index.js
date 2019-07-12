function callCreateShares() {
	let secret = document.getElementById("secret").value;
	let nShares = document.getElementById("nShares").value;
	let nNeededShares = document.getElementById("nNeededShares").value;
	let p = document.getElementById("p").value;
	console.log(p)
	
	let r = createShares(nNeededShares, nShares, p, secret);
	console.log("r", r);
}
