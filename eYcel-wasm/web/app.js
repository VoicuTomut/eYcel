import init, { encrypt, decrypt, decrypt_as_csv, validate_rules } from './pkg/eycel_wasm.js';

// ── State ───────────────────────────────────────────────────────
let wasmReady = false;
let encryptedResult = null;  // { encrypted_bytes, rules_yaml }
let decryptFileBytes = null;
let decryptFileName = '';
let decryptRulesText = null;
let decryptedResult = null;

// ── Init WASM ───────────────────────────────────────────────────
async function initWasm() {
    try {
        await init();
        wasmReady = true;
        console.log('eYcel WASM loaded');
    } catch (e) {
        console.error('Failed to load WASM:', e);
        showStatus('encrypt-status', 'Failed to load encryption engine. Please reload.', 'error');
    }
}

initWasm();

// ── Tab Navigation ──────────────────────────────────────────────
document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        tab.classList.add('active');
        document.getElementById('tab-' + tab.dataset.tab).classList.add('active');
    });
});

// ── Utility Functions ───────────────────────────────────────────
function showStatus(id, message, type) {
    const el = document.getElementById(id);
    el.className = 'status ' + type;
    el.innerHTML = (type === 'loading' ? '<span class="spinner"></span>' : '') + message;
    el.hidden = false;
}

function hideStatus(id) {
    document.getElementById(id).hidden = true;
}

function readFileAsArrayBuffer(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(new Uint8Array(reader.result));
        reader.onerror = reject;
        reader.readAsArrayBuffer(file);
    });
}

function readFileAsText(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsText(file);
    });
}

function triggerDownload(data, filename, mimeType) {
    const blob = new Blob([data], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

function setupDropZone(dropId, inputId, onFile) {
    const zone = document.getElementById(dropId);
    const input = document.getElementById(inputId);

    zone.addEventListener('click', () => input.click());

    zone.addEventListener('dragover', (e) => {
        e.preventDefault();
        zone.classList.add('dragover');
    });

    zone.addEventListener('dragleave', () => {
        zone.classList.remove('dragover');
    });

    zone.addEventListener('drop', (e) => {
        e.preventDefault();
        zone.classList.remove('dragover');
        if (e.dataTransfer.files.length > 0) {
            onFile(e.dataTransfer.files[0], zone);
        }
    });

    input.addEventListener('change', () => {
        if (input.files.length > 0) {
            onFile(input.files[0], zone);
        }
    });
}

function markLoaded(zone, filename) {
    zone.classList.add('loaded');
    zone.querySelector('p').innerHTML = '<strong>' + filename + '</strong>';
}

function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1048576).toFixed(1) + ' MB';
}

// ── Encrypt Tab ─────────────────────────────────────────────────
setupDropZone('encrypt-drop', 'encrypt-file', async (file, zone) => {
    if (!wasmReady) {
        showStatus('encrypt-status', 'Engine still loading, please wait...', 'loading');
        return;
    }

    markLoaded(zone, file.name);
    document.getElementById('encrypt-results').hidden = true;
    showStatus('encrypt-status', 'Encrypting ' + file.name + '...', 'loading');

    try {
        const bytes = await readFileAsArrayBuffer(file);
        const scrambleNumbers = document.getElementById('scramble-numbers').checked;
        const scrambleDates = document.getElementById('scramble-dates').checked;
        const result = encrypt(bytes, file.name, scrambleNumbers, scrambleDates);

        encryptedResult = {
            encrypted_bytes: result.encrypted_bytes,
            rules_yaml: result.rules_yaml,
            original_name: file.name
        };

        const stem = file.name.replace(/\.(xlsx|xls|csv)$/i, '');

        // Show stats
        const statsEl = document.getElementById('encrypt-stats');
        statsEl.innerHTML = `
            <div class="stat-card">
                <div class="label">Original Size</div>
                <div class="value">${formatBytes(bytes.length)}</div>
            </div>
            <div class="stat-card">
                <div class="label">Encrypted Size</div>
                <div class="value">${formatBytes(result.encrypted_bytes.length)}</div>
            </div>
        `;

        showStatus('encrypt-status', 'Encryption complete! All formulas preserved.', 'success');
        document.getElementById('encrypt-results').hidden = false;

    } catch (e) {
        showStatus('encrypt-status', 'Encryption failed: ' + e.message, 'error');
    }
});

document.getElementById('dl-encrypted').addEventListener('click', () => {
    if (!encryptedResult) return;
    const stem = encryptedResult.original_name.replace(/\.(xlsx|xls|csv)$/i, '');
    triggerDownload(
        encryptedResult.encrypted_bytes,
        stem + '_encrypted.xlsx',
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
    );
});

document.getElementById('dl-rules').addEventListener('click', () => {
    if (!encryptedResult) return;
    const stem = encryptedResult.original_name.replace(/\.(xlsx|xls|csv)$/i, '');
    triggerDownload(
        encryptedResult.rules_yaml,
        stem + '_rules.yaml',
        'text/yaml'
    );
});

// ── Decrypt Tab ─────────────────────────────────────────────────
function checkDecryptReady() {
    document.getElementById('btn-decrypt').disabled =
        !(decryptFileBytes && decryptRulesText);
}

setupDropZone('decrypt-drop-file', 'decrypt-file', async (file, zone) => {
    markLoaded(zone, file.name);
    decryptFileBytes = await readFileAsArrayBuffer(file);
    decryptFileName = file.name;
    checkDecryptReady();
});

setupDropZone('decrypt-drop-rules', 'decrypt-rules', async (file, zone) => {
    markLoaded(zone, file.name);
    decryptRulesText = await readFileAsText(file);
    checkDecryptReady();
});

document.getElementById('btn-decrypt').addEventListener('click', async () => {
    if (!wasmReady || !decryptFileBytes || !decryptRulesText) return;

    document.getElementById('decrypt-results').hidden = true;
    showStatus('decrypt-status', 'Decrypting...', 'loading');

    try {
        decryptedResult = decrypt(decryptFileBytes, decryptRulesText, decryptFileName);
        showStatus('decrypt-status', 'Decryption complete! Formulas preserved.', 'success');
        document.getElementById('decrypt-results').hidden = false;
    } catch (e) {
        showStatus('decrypt-status', 'Decryption failed: ' + e.message, 'error');
    }
});

document.getElementById('dl-decrypted').addEventListener('click', () => {
    if (!decryptedResult) return;
    triggerDownload(
        decryptedResult,
        'restored.xlsx',
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
    );
});

document.getElementById('dl-decrypted-csv').addEventListener('click', () => {
    if (!decryptFileBytes || !decryptRulesText) return;
    try {
        const csvBytes = decrypt_as_csv(decryptFileBytes, decryptRulesText, decryptFileName);
        triggerDownload(csvBytes, 'restored.csv', 'text/csv');
    } catch (e) {
        showStatus('decrypt-status', 'CSV export failed: ' + e.message, 'error');
    }
});

// ── Validate Tab ────────────────────────────────────────────────
setupDropZone('validate-drop', 'validate-file', async (file, zone) => {
    if (!wasmReady) {
        showStatus('validate-status', 'Engine still loading...', 'loading');
        return;
    }

    markLoaded(zone, file.name);
    showStatus('validate-status', 'Validating...', 'loading');

    try {
        const text = await readFileAsText(file);
        const result = validate_rules(text);

        const resultsEl = document.getElementById('validate-results');

        if (result.valid) {
            hideStatus('validate-status');
            let html = '<p class="validate-ok">&#10003; Rules file is valid</p>';

            // Build columns table
            const columns = result.columns;
            if (columns && typeof columns === 'object') {
                const entries = Object.entries(columns);
                if (entries.length > 0) {
                    html += '<table class="columns-table">';
                    html += '<thead><tr><th>Column</th><th>Transform</th></tr></thead>';
                    html += '<tbody>';
                    for (const [name, transform] of entries) {
                        html += `<tr>
                            <td>${name}</td>
                            <td><span class="transform-badge ${transform}">${transform}</span></td>
                        </tr>`;
                    }
                    html += '</tbody></table>';
                }
            }

            resultsEl.innerHTML = html;
        } else {
            showStatus('validate-status', '', 'error');
            let html = '<p class="validate-fail">&#10007; Rules file is invalid</p>';
            if (result.errors) {
                html += '<ul>';
                for (const err of result.errors) {
                    html += '<li>' + err + '</li>';
                }
                html += '</ul>';
            }
            resultsEl.innerHTML = html;
        }

        resultsEl.hidden = false;

    } catch (e) {
        showStatus('validate-status', 'Validation error: ' + e.message, 'error');
    }
});
