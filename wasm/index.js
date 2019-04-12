function callCreateShares() {
	let secret = Number(document.getElementById("secret").value);
	let nShares = Number(document.getElementById("nShares").value);
	let nNeededShares = Number(document.getElementById("nNeededShares").value);
	let p = Number(document.getElementById("p").value);
	
	createShares(nNeededShares, nShares, p, secret);
}
