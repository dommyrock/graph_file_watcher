import './style.css'
import { Orb, Color,OrbEventType } from "@memgraph/orb";

type GraphNode = {
  id: number;
  name: string;
  type: string;
  family?: string;
};
type GraphEdge = {
  id: number;
  start: number;
  end: number;
  label?: string;
};
const nodes: GraphNode[] = [
  { id: 1, name: "House of the Dragon", type: "Show" },
  { id: 2, name: "Rhaenyra Targaryen", type: "Person", family: "Targaryen" },
  { id: 3, name: "Daemon Targaryen", type: "Person", family: "Targaryen" },
  { id: 4, name: "Viserys Targaryen", type: "Person", family: "Targaryen" },
  { id: 5, name: "Otto Hightower", type: "Person", family: "Hightower" },
  { id: 6, name: "Alicent Hightower", type: "Person", family: "Hightower" },
];
const edges: GraphEdge[] = [
  { id: 1, start: 2, end: 1 },
  { id: 2, start: 3, end: 1 },
  { id: 3, start: 4, end: 1 },
  { id: 4, start: 5, end: 1 },
  { id: 5, start: 6, end: 1 },
  { id: 6, start: 3, end: 4, label: "brother of" },
  { id: 7, start: 4, end: 3, label: "brother of" },
  { id: 8, start: 2, end: 4, label: "child of" },
  { id: 9, start: 6, end: 5, label: "child of" },
];

const container = document.getElementById("graph");
const orb = new Orb<GraphNode, GraphEdge>(container as HTMLElement);

const imageUrlByNodeId: any = {
  1: "https://static.hbo.com/2022-06/house-of-the-dragon-ka-1920.jpg",
  2: "https://static.hbo.com/2022-05/house-of-the-dragon-character-rhaenyra-512x512_0.jpg?w=512",
  3: "https://static.hbo.com/2022-05/house-of-the-dragon-character-daemon-512x512.jpg?w=512",
  4: "https://static.hbo.com/2022-05/house-of-the-dragon-character-viserys-512x512_0.jpg?w=512",
  5: "https://static.hbo.com/2022-05/house-of-the-dragon-character-otto-512x512.jpg?w=512",
  6: "https://static.hbo.com/2022-05/house-of-the-dragon-character-alicent-512x512_2.jpg?w=512",
};
const colorByFamily: any = {
  Targaryen: "#c51c1c",
  Hightower: "#1ead2a",
};

// Set default style for new nodes and new edges
orb.data.setDefaultStyle({
  getNodeStyle(node) {
    const imageUrl = imageUrlByNodeId[node.id];
    // Shared style properties for all the nodes
    const commonProperties = {
      size: 10,
      fontSize: 3,
      imageUrl,
      label: node.data.name,
    };

    // Specific style properties for nodes where ".type = 'Person'"
    if (node.data.type === "Person") {
      return {
        ...commonProperties,
        // Border color will be the color of the family
        borderColor: colorByFamily[node.data.family!],
        borderWidth: 0.9,
        size: 6,
      };
    }

    return commonProperties;
  },
  getEdgeStyle(edge) {
    // Using Orb.Color to easily generate darker colors below
    const familyColor = new Color(colorByFamily[edge.endNode.data.family!] ?? "#999999")
    return {
      color: familyColor,
      colorHover: familyColor.getDarkerColor(),
      colorSelected: familyColor.getDarkerColor(),
      fontSize: 3,
      fontColor: familyColor.getDarkerColor(),
      // Edges will "label" property will have 3x larger width
      width: edge.data.label ? 0.3 : 0.1,
      widthHover: 0.9,
      widthSelected: 0.9,
      label: edge.data.label,
    };
  },
});


// Initialize nodes and edges
orb.data.setup({ nodes, edges });

// Change view settings to enable the physics (graph is more alive), and to
// disable transparency of unselected/not hovered nodes/edges
orb.view.setSettings({
  simulation: {
    isPhysicsEnabled: true,
  },
  render: {
    contextAlphaOnEventIsEnabled: false,
  },
});

orb.events.on(OrbEventType.NODE_CLICK, (event) => {
  if (event.node.data.type === "Show") {
    // If it is a central "Show" node, we want to return all the nodes and
    // edges - we use merge
    orb.data.merge({ nodes, edges });
  } else {
    // Otherwise, remove the clicked node from the orb
    orb.data.remove({ nodeIds: [event.node.id] });
  }
  orb.view.render();
});

orb.events.on(OrbEventType.EDGE_CLICK, (event) => {
  // On edge click, we want to remove the clicked edge
  orb.data.remove({ edgeIds: [event.edge.id] });
  orb.view.render();
});

// Render and recenter the view
orb.view.render(() => {
  orb.view.recenter();
});