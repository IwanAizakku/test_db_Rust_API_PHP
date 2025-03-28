<?php
// Set cache control headers to avoid caching and ensure fresh data
header("Cache-Control: no-cache, no-store, must-revalidate");
header("Pragma: no-cache");
header("Expires: 0");

// Set content type to text/plain to display raw text (encrypted string)
header("Content-Type: text/plain"); 

// Load environment variables
$dotenv = file('.env', FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
if ($dotenv === false) {
    die("Error loading .env file.");
}

$env = [];
foreach ($dotenv as $line) {
    list($key, $value) = explode('=', $line, 2);
    $env[trim($key)] = trim($value);
}

$api_url = $env['API_URL'] ?? null;
$api_token = $env['API_TOKEN'] ?? null;

if (!$api_url || !$api_token) {
    die("Missing API URL or Token.");
}

// Step 1: Send Request to API
$ch = curl_init($api_url);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_HTTPHEADER, [
    'Authorization: Bearer ' . $api_token,
    'Content-Type: application/json',
    'Accept: application/json',  // Add Accept header to explicitly request JSON
]);

$response = curl_exec($ch);
$http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);

if ($response === false) {
    die("CURL request failed.");
}

// Step 2: Return Encrypted Data as Raw Response (DO NOT DECRYPT HERE)
echo $response; // This ensures that the encrypted string is returned as-is
exit;
