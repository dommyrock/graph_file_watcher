import './style.css'
import { Orb, Color, OrbEventType } from "@memgraph/orb";
import edgesData from "../public/data/edges.json"
import nodesData from "../public/data/nodes.json"
//Style Defs -->  INodeStyle & IEdgeStyle
//D:\Me\Git\graph_file_watcher\frontend\graph-ui-ts\node_modules\@memgraph\orb\dist\models\style.d.ts

type GraphNode = {
  id: number;
  name: string;
  kind: string; //type
};
type GraphEdge = {
  id: number;
  start: number;
  end: number;
  label?: string;
};
const nodes: GraphNode[] = JSON.parse(nodesData);
const edges: GraphEdge[] = JSON.parse(edgesData);

const container = document.getElementById("graph");
const orb = new Orb<GraphNode, GraphEdge>(container as HTMLElement);

const imageUrlByNodeId: any = {
  0: "/folder.svg",
  1: "/doc.svg",
};
const colorByType: any = {
  Folder: "#c51c1c",
  File: "#1ead2a",
};

// Set default style for new nodes and new edges
orb.data.setDefaultStyle({
  getNodeStyle(node) {
    const imageUrl = imageUrlByNodeId[0];
    // Shared style properties for all the nodes
    const commonProperties = {
      size: 10,
      fontSize: 3,
      // shape:NodeShapeType.SQUARE,
      imageUrl,
      label: node.data.name,
    };

    // Specific style properties for nodes where ".type = 'File'"
    if (node.data.kind === "File") {
      return {
        ...commonProperties,
        // Border color will be the color of the family
        borderColor: colorByType[node.data.kind!],
        imageUrl:imageUrlByNodeId[1],
        borderWidth: 0.9,
        size: 5,
      };
    }

    return commonProperties;
  },
  getEdgeStyle(edge) {
    // Using Orb.Color to easily generate darker colors below
    const familyColor = new Color(colorByType[edge.endNode.data.kind!] ?? "#999999")
    return {
      color: familyColor,
      colorHover: familyColor.getDarkerColor(),
      colorSelected: familyColor.getDarkerColor(),
      fontSize: 3,
      fontColor: familyColor.getDarkerColor(),
      // Edges will "label" property will have 3x larger width
      width:  0.1,
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
    // isPhysicsEnabled: true,
  },
  render: {
    contextAlphaOnEventIsEnabled: false,
  },
});

orb.events.on(OrbEventType.NODE_CLICK, (event) => {
  if (event.node.data.kind === "Folder") {
    // If it is a central "Show" node, we want to return all the nodes and
    // edges - we use merge
    orb.data.merge({ nodes, edges });
  } else {
    // Otherwise, remove the clicked node from the orb
    // orb.data.remove({ nodeIds: [event.node.id] }); //DISABLED while testing
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