cs ts:
    interface User {
        id: number;
        name: string;
        email: string;
        age: number;
    }
    
    export function filterAdults(users: User[]): User[] {
        return users.filter(user => user.age >= 18);
    }
    
    export function getUserNames(users: User[]): string[] {
        return users.map(user => user.name);
    }

mn main():
    users = [
        {id: 1, name: "Alice", email: "alice@example.com", age: 30},
        {id: 2, name: "Bob", email: "bob@example.com", age: 17},
        {id: 3, name: "Charlie", email: "charlie@example.com", age: 25}
    ]
    
    adults = ts.filterAdults(users)
    print("Adults:", adults)
    
    names = ts.getUserNames(adults)
    print("Names:", names)
