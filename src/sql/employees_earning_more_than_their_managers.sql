SELECT employee.name AS Employee
FROM Employee AS employee
         INNER JOIN Employee AS manager ON employee.managerId = manager.id
WHERE manager.salary < employee.salary;