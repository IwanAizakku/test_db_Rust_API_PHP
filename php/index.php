<?php
session_start();
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

$api_url = $env['API_URL'] ?? null;
$api_token = $env['API_TOKEN'] ?? null;
$sodium_key_base64 = $env['SODIUM_KEY'] ?? null;

if (!$api_url || !$api_token || !$sodium_key_base64) {
    die("Missing API credentials in .env file.");
}

// Decode the Base64-encoded Sodium key
$sodium_key = base64_decode($sodium_key_base64);
if ($sodium_key === false || strlen($sodium_key) !== SODIUM_CRYPTO_SECRETBOX_KEYBYTES) {
    die("Invalid Sodium encryption key.");
}

// Function to send API requests
function send_api_request($data) {
    global $api_url, $api_token;

    $ch = curl_init($api_url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode($data));
    curl_setopt($ch, CURLOPT_HTTPHEADER, [
        'Authorization: Bearer ' . $api_token,
        'Content-Type: application/json',
        'Accept: application/json',
    ]);

    $response = curl_exec($ch);
    curl_close($ch);

    return $response;
}

// Handle form submissions (Create, Update, Delete)
if ($_SERVER["REQUEST_METHOD"] === "POST") {
    if (isset($_POST['create'])) {
        $payload = [
            "action" => "create",
            "employee" => [
                "emp_no" => (int) $_POST['emp_no'],
                "birth_date" => $_POST['birth_date'],
                "first_name" => $_POST['first_name'],
                "last_name" => $_POST['last_name'],
                "gender" => $_POST['gender'],
                "hire_date" => $_POST['hire_date']
            ]
        ];
        $response = send_api_request($payload);
        $_SESSION['message'] = $response ? "Employee added successfully!" : "Error adding employee.";
        $_SESSION['alert_type'] = $response ? "success" : "danger";
    } elseif (isset($_POST['update'])) {
        $payload = [
            "action" => "update",
            "emp_no" => (int) $_POST['emp_no'],
            "employee" => [
                "emp_no" => (int) $_POST['emp_no'],
                "birth_date" => $_POST['birth_date'],
                "first_name" => $_POST['first_name'],
                "last_name" => $_POST['last_name'],
                "gender" => $_POST['gender'],
                "hire_date" => $_POST['hire_date']
            ]
        ];
        $response = send_api_request($payload);
        $_SESSION['message'] = $response ? "Employee updated successfully!" : "Error updating employee.";
        $_SESSION['alert_type'] = $response ? "success" : "danger";
    } elseif (isset($_POST['delete'])) {
        $payload = [
            "action" => "delete",
            "emp_no" => (int) $_POST['emp_no']
        ];
        $response = send_api_request($payload);
        $_SESSION['message'] = $response ? "Employee deleted successfully!" : "Error deleting employee.";
        $_SESSION['alert_type'] = $response ? "success" : "danger";
    }

    header("Location: index.php");
    exit();
}

// Fetch encrypted data from API
$api_data = json_encode(['action' => 'read_all']);
$encrypted_data = send_api_request(json_decode($api_data, true));

if ($encrypted_data === false) {
    die("Failed to fetch encrypted data.");
}

// Decrypt response
$decoded_data = base64_decode($encrypted_data);
if ($decoded_data === false) {
    die("Failed to decode base64 response.");
}

$nonce_size = SODIUM_CRYPTO_SECRETBOX_NONCEBYTES;
if (strlen($decoded_data) < $nonce_size) {
    die("Invalid encrypted data format.");
}

$nonce = substr($decoded_data, 0, $nonce_size);
$ciphertext = substr($decoded_data, $nonce_size);

$plaintext = sodium_crypto_secretbox_open($ciphertext, $nonce, $sodium_key);
if ($plaintext === false) {
    die("Decryption failed.");
}

$employees = json_decode($plaintext, true);
if (!is_array($employees)) {
    die("Invalid JSON data after decryption.");
}

header("X-Encrypted-Data: " . htmlspecialchars($encrypted_data));
ob_end_flush();
?>

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css">
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
    <title>Employee List</title>
</head>
<body>
    <!-- ALERT MESSAGES -->
    <div id="alert-container" class="position-fixed top-0 start-50 translate-middle-x mt-3" style="z-index: 1050; width: 50%;">
    </div>

    <div class="container">
        <header class="d-flex justify-content-between my-4">
            <h1>Employee List</h1>
            <button class="btn btn-success" data-bs-toggle="modal" data-bs-target="#addEmployeeModal">Add Employee</button>
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
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
            <?php if (is_array($employees) && isset($employees[0])): ?>
                <?php foreach ($employees as $employee): ?>
                    <tr>
                        <td><?= htmlspecialchars($employee['emp_no']) ?></td>
                        <td><?= htmlspecialchars($employee['birth_date']) ?></td>
                        <td><?= htmlspecialchars($employee['first_name']) ?></td>
                        <td><?= htmlspecialchars($employee['last_name']) ?></td>
                        <td><?= htmlspecialchars($employee['gender']) ?></td>
                        <td><?= htmlspecialchars($employee['hire_date']) ?></td>
                        <td>
                            <form method="POST" class="d-inline">
                                <input type="hidden" name="emp_no" value="<?= htmlspecialchars($employee['emp_no']) ?>">
                                <button type="submit" name="delete" class="btn btn-danger btn-sm">Delete</button>
                            </form>
                            <button type="button" class="btn btn-warning btn-sm" data-bs-toggle="modal" data-bs-target="#editEmployeeModal" 
                                onclick="fillUpdateForm(<?= htmlspecialchars(json_encode($employee)) ?>)">Edit</button>
                        </td>
                    </tr>
                <?php endforeach; ?>
            <?php else: ?>
                <tr><td colspan='7'>No employees to display.</td></tr>
            <?php endif; ?>
            </tbody>
        </table>
    </div>

    <!-- ADD EMPLOYEE MODAL -->
    <div class="modal fade" id="addEmployeeModal" tabindex="-1" aria-labelledby="addEmployeeModalLabel" aria-hidden="true">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="addEmployeeModalLabel">Add New Employee</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <form method="POST">
                        <label>Employee No</label>
                        <input type="number" name="emp_no" class="form-control mb-2" required>
                        
                        <label>Birth Date</label>
                        <input type="date" name="birth_date" class="form-control mb-2" required>
                        
                        <label>First Name</label>
                        <input type="text" name="first_name" class="form-control mb-2" required>
                        
                        <label>Last Name</label>
                        <input type="text" name="last_name" class="form-control mb-2" required>
                        
                        <label>Gender</label>
                        <select name="gender" class="form-control mb-2" required>
                            <option value="M">Male</option>
                            <option value="F">Female</option>
                        </select>
                        
                        <label>Hire Date</label>
                        <input type="date" name="hire_date" class="form-control mb-2" required>
                        
                        <button type="submit" name="create" class="btn btn-success w-100">Add Employee</button>
                    </form>
                </div>
            </div>
        </div>
    </div>

    <!-- EDIT EMPLOYEE MODAL -->
    <div class="modal fade" id="editEmployeeModal" tabindex="-1" aria-labelledby="editEmployeeModalLabel" aria-hidden="true">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="editEmployeeModalLabel">Edit Employee</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <form method="POST" id="updateForm">
                        <input type="hidden" name="emp_no" id="update_emp_no">
                        <label>Birth Date</label>
                        <input type="date" name="birth_date" id="update_birth_date" class="form-control mb-2" required>
                        <label>First Name</label>
                        <input type="text" name="first_name" id="update_first_name" class="form-control mb-2" required>
                        <label>Last Name</label>
                        <input type="text" name="last_name" id="update_last_name" class="form-control mb-2" required>
                        <label>Gender</label>
                        <select name="gender" id="update_gender" class="form-control mb-2" required>
                            <option value="M">Male</option>
                            <option value="F">Female</option>
                        </select>
                        <label>Hire Date</label>
                        <input type="date" name="hire_date" id="update_hire_date" class="form-control mb-2" required>
                        <button type="submit" name="update" class="btn btn-primary w-100">Update</button>
                    </form>
                </div>
            </div>
        </div>
    </div>

    <!-- JavaScript to Fill the Edit Employee Form -->
    <script>
    function fillUpdateForm(employee) {
        document.getElementById('update_emp_no').value = employee.emp_no;
        document.getElementById('update_birth_date').value = employee.birth_date;
        document.getElementById('update_first_name').value = employee.first_name;
        document.getElementById('update_last_name').value = employee.last_name;
        document.getElementById('update_gender').value = employee.gender;
        document.getElementById('update_hire_date').value = employee.hire_date;
    }
    function showAlert(message, type = "success") {
        let alertContainer = document.getElementById("alert-container");

        let alert = document.createElement("div");
        alert.className = `alert alert-${type} alert-dismissible fade show text-center`;
        alert.role = "alert";
        alert.innerHTML = `
            ${message}
            <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
        `;

        alertContainer.appendChild(alert);

        // Remove alert after 3 seconds
        setTimeout(() => {
            alert.remove();
        }, 3000);
    }

    // Show alerts based on PHP messages
    document.addEventListener("DOMContentLoaded", function() {
        <?php if (isset($_SESSION['message'])): ?>
            showAlert("<?= $_SESSION['message'] ?>", "<?= $_SESSION['alert_type'] ?? 'success' ?>");
            <?php unset($_SESSION['message'], $_SESSION['alert_type']); ?>
        <?php endif; ?>
    });

    // Confirm before deletion
    document.addEventListener("DOMContentLoaded", function() {
        const deleteForms = document.querySelectorAll("form button[name='delete']");

        deleteForms.forEach(button => {
            button.addEventListener("click", function(event) {
                const confirmed = confirm("Are you sure you want to delete this employee?");
                if (!confirmed) {
                    event.preventDefault(); // Stop form submission
                }
            });
        });
    });
    </script>
</body>
</html>
