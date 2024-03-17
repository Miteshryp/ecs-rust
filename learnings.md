
### Why can't we attach system (handlers) to the component managers

The system needs the following attributes as argument into its methods for proper functioning
- Owner World's API access.
- List of components of similar type for handling interactions.
- entity_id access for the owner entity.
- current component to be acted on

Now, if system is bounded into the component manager, the world API would end up containing the managers. This leads to a problem where we are unable to pass the parent structure as an argument in the member functions because the `&mut self` cannot have multiple instances.
If we try to solve this using just `self`, we still run into another issue where we are still not able to pass the `World` into the member function because `self` is partially decomposed (because we accessed entity_manager), hence the world struct is not bound to be valid.

The solution to this problem is to distinctly seperate out the state of the world from its logic, and pass the state into the logic during updates


