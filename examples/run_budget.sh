#!/usr/bin/env bash
# run_budget.sh — compile budget.va to C and run it
# Usage: bash examples/run_budget.sh
set -e

COMPILER="./target/release/valea"
SOURCE="examples/budget.va"
OUT="/tmp/budget_main.c"

# 1. Validate the source first
echo "Checking $SOURCE ..."
$COMPILER check "$SOURCE"

# 2. Emit C from Valea
echo "Emitting C ..."
$COMPILER emit-c "$SOURCE" > /tmp/budget_gen.c

# 3. Append a main() that calls the generated functions and prints a report
cat > "$OUT" << 'EOF'
/* ── auto-generated from budget.va ── */
EOF
cat /tmp/budget_gen.c >> "$OUT"
cat >> "$OUT" << 'EOF'

#include <stdio.h>

static void line() { printf("  %-26s\n", "──────────────────────────"); }

int main(void) {
    long income     = total_income();
    long expenses   = total_expenses();
    long net        = income - expenses;

    printf("\n╔══════════════════════════════╗\n");
    printf("║     Monthly Budget Report    ║\n");
    printf("╚══════════════════════════════╝\n\n");

    printf("  INCOME\n");
    printf("  %-20s  $%5ld\n", "Salary",       salary());
    printf("  %-20s  $%5ld\n", "Consulting",   consulting());
    printf("  %-20s  $%5ld\n", "Side project", side_project());
    line();
    printf("  %-20s  $%5ld\n\n", "Total income", income);

    printf("  EXPENSES\n");
    printf("  %-20s  $%5ld\n", "Rent",          rent());
    printf("  %-20s  $%5ld\n", "Groceries",     groceries());
    printf("  %-20s  $%5ld\n", "Transport",     transport());
    printf("  %-20s  $%5ld\n", "Utilities",     utilities());
    printf("  %-20s  $%5ld\n", "Phone",         phone());
    printf("  %-20s  $%5ld\n", "Subscriptions", subscriptions());
    printf("  %-20s  $%5ld\n", "Entertainment", entertainment());
    line();
    printf("  %-20s  $%5ld\n\n", "Total expenses", expenses);

    printf("  %-20s  $%5ld  %s\n\n",
        "NET SAVINGS", net,
        net >= 0 ? "✓ surplus" : "✗ deficit");

    return net >= 0 ? 0 : 1;
}
EOF

# 4. Compile with gcc
echo "Compiling ..."
gcc "$OUT" -o /tmp/budget

# 5. Run it
echo ""
/tmp/budget
