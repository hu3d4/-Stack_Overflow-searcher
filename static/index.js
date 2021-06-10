function addHistory() {
	document.forms["addHistory"].submit();
}

function customSearch() {
	let input_text = document.getElementById("input").value;
	document.forms["customSearch"].elements["q"].value = input_text;
	document.forms["customSearch"].submit();
}

function callAfter() {
	let acc = [customSearch(), addHistory()];
	while (acc.length > 0) {
		acc.shift();
	}
}

let auth0 = null;

const fetchAuthConfig = () => fetch("/auth_config.json");

const configureClient = async () => {
	const response = await fetchAuthConfig();
	const config = await response.json();

	auth0 = await createAuth0Client({
		domain: config.domain,
		client_id: config.clientId,
	});
};

window.onload = async () => {
	await configureClient();

	updateUI();

	const isAuthenticated = await auth0.isAuthenticated();
	if (isAuthenticated) {
		return;
	}

	const query = window.location.search;
	if (query.includes("code=") && query.includes("state=")) {
		await auth0.handleRedirectCallback();

		updateUI();

		window.history.replaceState({}, document.title, "/");
	}
};

const updateUI = async () => {
	const isAuthenticated = await auth0.isAuthenticated();

	document.getElementById("btn-logout").disabled = !isAuthenticated;
	document.getElementById("btn-login").disabled = isAuthenticated;

	if (isAuthenticated) {
		document.getElementById("gated-content").classList.remove("hidden");
		const jsonData = JSON.stringify(await auth0.getUser());
		const jsonObject = JSON.parse(jsonData);
		const getUserSubjectJson = JSON.stringify(await jsonObject.sub);
		document.getElementById("ipt-user-profile").value = getUserSubjectJson;

		// ユーザーに合わせてすべての履歴を削除するaction属性
		document.DeleteAll.action = `/delete/${getUserSubjectJson}`;

		const getUserSubjectValue =
			document.getElementById("ipt-user-profile").value;
		const elementsGroup = document.getElementsByClassName("history__group");

		for (let i = 0; i < elementsGroup.length; i++) {
			if (
				elementsGroup.item(i).children[1].textContent.trim() !=
				getUserSubjectValue
			) {
				elementsGroup[i].style.display = "none";
			}
		}
	} else {
		document.getElementById("gated-content").classList.add("hidden");
	}
};

const login = async () => {
	await auth0.loginWithRedirect({
		redirect_uri: window.location.origin,
	});
};

const logout = () => {
	auth0.logout({
		returnTo: window.location.origin,
	});
};
