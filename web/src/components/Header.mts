// export function Header() {
//   return (
//     <div className="flex h-16 items-center justify-between bg-gray-100 px-2">
//       <span className="text-2xl font-bold text-violet-400">ALC-LISP</span>
//     </div>
//   );
// }

// class Counter {
//   private counter: number = 0;

//   addCounter(x: number) {
//     this.counter += x;
//     return this.counter;
//   }

//   subCounter(x: number) {
//     this.counter -= x;
//     return this.counter;
//   }
// }

// function createCounterFunction() {
//   let counter = 0;

//   const addFn = function (x: number) {
//     counter += x;
//     return counter;
//   };

//   const subFn = function (x: number) {
//     counter -= x;
//     return counter;
//   };

//   return [addFn, subFn];
// }

// const [addCounterFn, subCounterFn] = createCounterFunction();

// console.log(addCounterFn(2));
// console.log(addCounterFn(2));
// console.log(addCounterFn(6));

// console.log(subCounterFn(5));

abstract class Entity {
  Name(): string {
    return "Entity";
  }
  Destroy(): boolean {
    return true;
  }
}

interface Interactable {
  interact(source: Entity): boolean;
}

function isInteractable(node:Entity) : node is Entity & Interactable {
  if ("interact" in node && typeof node.interact === 'function') {
    return true;
  }
  return false
}

interface Petable {
  pet(source: Entity): boolean;
}

class Door extends Entity implements Interactable,Petable {
  private open: boolean = false;

  override Name() :string{
    return "Door";
  }

  interact(source: Entity): boolean {
    this.open = !this.open

    console.log(`[${source.Name()}] opens the door`)

    return true;
  }

  pet(_source: Entity): boolean {
    console.log("the door purrrs  and says meow meow")

    return true;
  }
}

class Player extends Entity {
  input(): void {
    // codigo que lida com a input
    // if player presses interact
    const entity: Entity = new Door();
    // Seria a forma godot
    //entity.has_method("interact");


    if (isInteractable(entity)) {
      entity.interact(this);
    }
  }
}
