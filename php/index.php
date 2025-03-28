<?php
// Start output buffering to control the order of output
ob_start();

// Load environment variables manually
$dotenv = file('.env', FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
if ($dotenv === false) {
    die("Error loading .env file.");
}

$env = [];
foreach ($dotenv as $line) {
    list($key, $value) = explode('=', $line, 2);
    $env[trim($key)] = trim($value);
}

$sodium_key_base64 = $env['SODIUM_KEY'] ?? null;
if (!$sodium_key_base64) {
    die("Missing SODIUM_KEY in .env file.");
}

// Decode the Base64-encoded Sodium key
$sodium_key = base64_decode($sodium_key_base64);
if ($sodium_key === false || strlen($sodium_key) !== SODIUM_CRYPTO_SECRETBOX_KEYBYTES) {
    die("Invalid Sodium encryption key.");
}

// Step 1: Fetch encrypted data from api_request.php
$encrypted_data = file_get_contents("http://localhost/test_db/php/api_request.php");
if ($encrypted_data === false) {
    die("Failed to fetch encrypted data.");
}

// Step 2: Decode base64 response
$decoded_data = base64_decode($encrypted_data);
if ($decoded_data === false) {
    die("Failed to decode base64 response.");
}

// Step 3: Extract Nonce & Decrypt
$nonce_size = SODIUM_CRYPTO_SECRETBOX_NONCEBYTES;
if (strlen($decoded_data) < $nonce_size) {
    die("Invalid encrypted data format.");
}

$nonce = substr($decoded_data, 0, $nonce_size);
$ciphertext = substr($decoded_data, $nonce_size);

// Decrypt data
$plaintext = sodium_crypto_secretbox_open($ciphertext, $nonce, $sodium_key);
if ($plaintext === false) {
    die("Decryption failed.");
}

// Decode the decrypted data (JSON format)
$employees = json_decode($plaintext, true);
if (!is_array($employees)) {
    die("Invalid JSON data after decryption.");
}

// Send the encrypted data in a custom HTTP header (not visible in HTML source)
header("X-Encrypted-Data: " . htmlspecialchars($encrypted_data));

// Flush the output buffer (sends everything before the HTML)
ob_end_flush();

// Now we render the webpage with decrypted employee data
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css">
    <title>Employee List</title>
</head>
<body>
    <div class="container">
        <header class="d-flex justify-content-between my-4">
            <h1>Employee List</h1>
        </header>
        <table class="table table-bordered">
            <thead>
                <tr>
                    <th>Employee No</th>
                    <th>Birth Date</th>
                    <th>First Name</th>
                    <th>Last Name</th>
                    <th>Gender</th>
                    <th>Hire Date</th>
                </tr>
            </thead>
            <tbody>
            <?php foreach ($employees as $employee): ?>
                <tr>
                    <td><?= htmlspecialchars($employee['emp_no']) ?></td>
                    <td><?= htmlspecialchars($employee['birth_date']) ?></td>
                    <td><?= htmlspecialchars($employee['first_name']) ?></td>
                    <td><?= htmlspecialchars($employee['last_name']) ?></td>
                    <td><?= htmlspecialchars($employee['gender']) ?></td>
                    <td><?= htmlspecialchars($employee['hire_date']) ?></td>
                </tr>
            <?php endforeach; ?>
            </tbody>
        </table>
    </div>
</body>
</html>
